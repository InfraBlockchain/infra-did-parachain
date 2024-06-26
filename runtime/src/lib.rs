#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

mod constants;
pub use constants::{currency::*, fee::WeightToFee};

mod weights;
pub mod xcm_config;
use xcm_config::{XcmRouter, UniversalLocation, NativeAssetsPalletLocation};
use xcm::latest::{BodyId, MultiLocation};
use cumulus_pallet_parachain_system::RelayNumberStrictlyIncreases;
use parachains_common::{
	constants::*, impls::DealWithFees, infra_relay::consensus::*, opaque::*, AccountId, AuraId,
	Balance, BlockNumber, Hash, Nonce, Signature,
};
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{
        AccountIdLookup, BlakeTwo256, Block as BlockT, ConvertInto, TryConvertInto as JustTry, AccountIdConversion
    },
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult,
};

use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use frame_support::{
	construct_runtime,
	dispatch::DispatchClass,
	parameter_types,
	traits::{
		tokens::fungibles::{Balanced, Credit, UnionOf},
		AsEnsureOriginWithArg, ConstBool, ConstU32, ConstU64, ConstU8, EitherOfDiverse,
		InstanceFilter,
	},
	weights::{ConstantMultiplier, Weight},
	PalletId,
};
use frame_system::{
    limits::{BlockLength, BlockWeights},
    EnsureRoot, EnsureSigned,
};
use pallet_system_token_tx_payment::{TransactionFeeCharger, HandleCredit};
pub use sp_runtime::{
    infra::*,
    MultiAddress, Perbill, Permill,
};
use xcm_config::{XcmConfig, XcmOriginToTransactDispatchOrigin};
use infra_asset_common::{
	local_and_foreign_assets::LocalFromLeft, AssetIdForNativeAssets, AssetIdForNativeAssetsConvert,
};

#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

// Polkadot imports
use runtime_common::BlockHashCount;

use weights::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight};

pub use did_core::{
    accumulator, anchor, attest, blob, common, did,
    offchain_signatures::{self, BBSPlusPublicKey, OffchainPublicKey, PSPublicKey},
    revoke, status_list_credential, trusted_entity,
};

// XCM Imports
use xcm_executor::XcmExecutor;

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_system_token_tx_payment::ChargeSystemToken<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
    generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra>;
pub type Migrate = ();

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    Migrate,
>;

impl_opaque_keys! {
    pub struct SessionKeys {
        pub aura: Aura,
    }
}

#[cfg(not(feature = "std"))]
mod wasm_handlers {
    #[panic_handler]
    #[no_mangle]
    pub fn panic(info: &core::panic::PanicInfo) -> ! {
        let message = sp_std::alloc::format!("{}", info);
        log::error!("{}", message);
        ::core::arch::wasm32::unreachable();
    }

    #[cfg(enable_alloc_error_handler)]
    #[alloc_error_handler]
    pub fn oom(_: core::alloc::Layout) -> ! {
        log::error!("Runtime memory exhausted. Aborting");
        ::core::arch::wasm32::unreachable();
    }
}

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("InfraBlockchain DID Parachain"),
    impl_name: create_runtime_str!("InfraBlockchain DID Parachain"),
    authoring_version: 1,
    spec_version: 10_000,
    impl_version: 0,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 13,
    state_version: 10,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

parameter_types! {
    pub const DepositToCreateAsset: Balance = 1 * DOLLARS;
    pub const DepositToMaintainAsset: Balance = deposit(1, 16);
    pub const ApprovalDeposit: Balance = EXISTENTIAL_DEPOSIT;
    pub const StringLimit: u32 = 50;
    /// Key = 32 bytes, Value = 36 bytes (32+1+1+1+1)
    pub const MetadataDepositBase: Balance = deposit(1, 68);
    pub const MetadataDepositPerByte: Balance = deposit(0, 1);
}

/// We allow root and the Relay Chain council to execute privileged asset operations.
pub type RootOrigin = EnsureRoot<AccountId>;

pub type NativeAssetsInstance = pallet_assets::Instance1;
type NativeAssetsCall = pallet_assets::Call<Runtime, NativeAssetsInstance>;
impl pallet_assets::Config<NativeAssetsInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = AssetIdForNativeAssets;
	type AssetIdParameter = codec::Compact<AssetIdForNativeAssets>;
	type SystemTokenWeight = SystemTokenWeight;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
	type ForceOrigin = RootOrigin;
	type AssetDeposit = DepositToCreateAsset;
	type AssetAccountDeposit = DepositToMaintainAsset;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type CallbackHandle = ();
	type WeightInfo = ();
	type RemoveItemsLimit = ConstU32<1000>;
}

pub type ForeignAssetsInstance = pallet_assets::Instance2;
type ForeignAssetsCall = pallet_assets::Call<Runtime, ForeignAssetsInstance>;
impl pallet_assets::Config<ForeignAssetsInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = xcm::v3::MultiLocation;
	type AssetIdParameter = xcm::v3::MultiLocation;
	type SystemTokenWeight = SystemTokenWeight;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
	type ForceOrigin = RootOrigin; //TODO
	type AssetDeposit = DepositToCreateAsset;
	type AssetAccountDeposit = DepositToMaintainAsset;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type CallbackHandle = ();
	type WeightInfo = ();
	type RemoveItemsLimit = ConstU32<1000>;
}

pub struct ReanchorHandler;
impl ReanchorSystemToken<MultiLocation> for ReanchorHandler {
	type Error = ();
	fn reanchor_system_token(l: &mut MultiLocation) -> Result<(), Self::Error> {
		let target = MultiLocation::parent();
		let context = UniversalLocation::get();
		l.reanchor(&target, context).map_err(|_| {})?;
		Ok(())
	}
}

/// Union fungibles implementation for `Assets` and `ForeignAssets`.
pub type NativeAndForeignAssets = UnionOf<
	Assets,
	ForeignAssets,
	LocalFromLeft<
		AssetIdForNativeAssetsConvert<NativeAssetsPalletLocation>,
		AssetIdForNativeAssets,
		xcm::v3::MultiLocation,
	>,
	xcm::v3::MultiLocation,
	AccountId,
	ReanchorHandler,
>;

parameter_types! {
    pub const FeeTreasuryId: PalletId = PalletId(*b"infrapid");
}

pub struct BootstrapCallFilter;
impl frame_support::traits::Contains<RuntimeCall> for BootstrapCallFilter {
    #[cfg(not(feature = "fast-runtime"))]
    fn contains(call: &RuntimeCall) -> bool {
        match call {
            RuntimeCall::Assets(
                pallet_assets::Call::create { .. }
                | pallet_assets::Call::set_metadata { .. }
                | pallet_assets::Call::mint { .. },
            )
            | RuntimeCall::InfraXcm(pallet_xcm::Call::limited_teleport_assets { .. })
            | RuntimeCall::InfraParaCore(
                cumulus_pallet_infra_parachain_core::Call::request_register_system_token { .. },
            ) => true,
            _ => false,
        }
    }
    #[cfg(feature = "fast-runtime")]
    fn contains(call: &RuntimeCall) -> bool {
        match call {
            _ => true,
        }
    }
}

impl pallet_system_token_tx_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SystemConfig = InfraParaCore;
	type PoTHandler = ParachainSystem;
	type Fungibles = NativeAndForeignAssets;
	type OnChargeSystemToken =
		TransactionFeeCharger<Runtime, SystemTokenConversion, CreditToBucket>;
	type BootstrapCallFilter = BootstrapCallFilter;
	type PalletId = FeeTreasuryId;
}

pub struct CreditToBucket;
impl HandleCredit<AccountId, NativeAndForeignAssets> for CreditToBucket {
	fn handle_credit(credit: Credit<AccountId, NativeAndForeignAssets>) {
		let dest = FeeTreasuryId::get().into_account_truncating();
		let _ = <NativeAndForeignAssets as Balanced<AccountId>>::resolve(&dest, credit);
	}
}

impl pallet_system_token_conversion::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = SystemTokenBalance;
	type AssetKind = xcm::v3::MultiLocation;
	type Fungibles = NativeAndForeignAssets;
	type SystemConfig = InfraParaCore;
}

parameter_types! {
    pub const Version: RuntimeVersion = VERSION;

    // This part is copied from Substrate's `bin/node/runtime/src/lib.rs`.
    //  The `RuntimeBlockLength` and `RuntimeBlockWeights` exist here because the
    // `DeletionWeightLimit` and `DeletionQueueDepth` depend on those to parameterize
    // the lazy contract deletion.
    pub RuntimeBlockLength: BlockLength =
        BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
        .base_block(BlockExecutionWeight::get())
        .for_class(DispatchClass::all(), |weights| {
            weights.base_extrinsic = ExtrinsicBaseWeight::get();
        })
        .for_class(DispatchClass::Normal, |weights| {
            weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
        })
        .for_class(DispatchClass::Operational, |weights| {
            weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
            // Operational transactions have some extra reserved space, so that they
            // are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
            weights.reserved = Some(
                MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
            );
        })
        .avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
        .build_or_panic();
    pub const SS58Prefix: u16 = 42;
}

// Configure FRAME pallets to include in runtime.
impl frame_system::Config for Runtime {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = RuntimeBlockWeights;
    type BlockLength = RuntimeBlockLength;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = Nonce;
    type Hash = Hash;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = AccountIdLookup<AccountId, ()>;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type DbWeight = RocksDbWeight;
    type Version = Version;
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = weights::frame_system::WeightInfo<Runtime>;
    type SS58Prefix = SS58Prefix;
    type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type WeightInfo = ();
}

impl pallet_timestamp::Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Aura;
    type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
    type WeightInfo = ();
}

impl pallet_authorship::Config for Runtime {
    type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
    type EventHandler = (CollatorSelection,);
}

parameter_types! {
    pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
}

impl pallet_balances::Config for Runtime {
    type MaxLocks = ConstU32<50>;
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = weights::pallet_balances::WeightInfo<Runtime>;
    type MaxReserves = ConstU32<50>;
    type ReserveIdentifier = [u8; 8];
    type RuntimeHoldReason = RuntimeHoldReason;
    type FreezeIdentifier = ();
    type MaxHolds = ConstU32<0>;
    type MaxFreezes = ConstU32<0>;
}

parameter_types! {
    /// Relay Chain `TransactionByteFee` / 10
    pub const TransactionByteFee: Balance = 1 * 100_000;
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction =
        pallet_transaction_payment::CurrencyAdapter<Balances, DealWithFees<Runtime>>;
    type WeightToFee = WeightToFee;
    type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
    type FeeMultiplierUpdate = ();
    type OperationalFeeMultiplier = ConstU8<5>;
}

parameter_types! {
    pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
    pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
}

type ConsensusHook = cumulus_pallet_aura_ext::FixedVelocityConsensusHook<
    Runtime,
    RELAY_CHAIN_SLOT_DURATION_MILLIS,
    BLOCK_PROCESSING_VELOCITY,
    UNINCLUDED_SEGMENT_CAPACITY,
>;

impl cumulus_pallet_parachain_system::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnSystemEvent = ();
	type SelfParaId = parachain_info::Pallet<Runtime>;
	type DmpMessageHandler = DmpQueue;
	type ReservedDmpWeight = ReservedDmpWeight;
	type OutboundXcmpMessageSource = XcmpQueue;
	type XcmpMessageHandler = XcmpQueue;
	type ReservedXcmpWeight = ReservedXcmpWeight;
	type UpdateRCConfig = InfraParaCore;
	type CheckAssociatedRelayNumber = RelayNumberStrictlyIncreases;
	type ConsensusHook = ConsensusHook;
}

parameter_types! {
    pub const ActiveRequestPeriod: BlockNumber = 100;
}

impl cumulus_pallet_infra_parachain_core::Config for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeEvent = RuntimeEvent;
	type SystemTokenId = MultiLocation;
	type UniversalLocation = UniversalLocation;
	type Fungibles = NativeAndForeignAssets;
	type ActiveRequestPeriod = ActiveRequestPeriod;
}

impl parachain_info::Config for Runtime {}

impl cumulus_pallet_aura_ext::Config for Runtime {}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type ChannelInfo = ParachainSystem;
    type VersionWrapper = ();
    type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
    type ControllerOrigin = EnsureRoot<AccountId>;
    type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
    type WeightInfo = ();
    type PriceForSiblingDelivery = ();
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
}

parameter_types! {
    pub const Period: u32 = 6 * HOURS;
    pub const Offset: u32 = 0;
}

impl pallet_session::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    // we don't have stash and controller, thus we don't need the convert as well.
    type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
    type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
    type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
    type SessionManager = CollatorSelection;
    // Essentially just Aura, but let's be pedantic.
    type SessionHandler = <SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
    type Keys = SessionKeys;
    type WeightInfo = ();
}

impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<100_000>;
    type AllowMultipleBlocksPerSlot = ConstBool<false>;
    #[cfg(feature = "experimental")]
    type SlotDuration = pallet_aura::MinimumPeriodTimesTwo<Self>;
}

parameter_types! {
    pub const PotId: PalletId = PalletId(*b"PotStake");
    pub const MaxCandidates: u32 = 1000;
    pub const MinCandidates: u32 = 5;
    pub const SessionLength: BlockNumber = 6 * HOURS;
    pub const MaxInvulnerables: u32 = 100;
}

// We allow root only to execute privileged collator selection operations.
pub type CollatorSelectionUpdateOrigin = EnsureRoot<AccountId>;

impl pallet_collator_selection::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type UpdateOrigin = CollatorSelectionUpdateOrigin;
    type PotId = PotId;
    type MaxCandidates = ConstU32<100>;
    type MinEligibleCollators = ConstU32<4>;
    type MaxInvulnerables = ConstU32<20>;
    // should be a multiple of session or things will get inconsistent
    type KickThreshold = Period;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
    type ValidatorRegistration = Session;
    type WeightInfo = weights::pallet_collator_selection::WeightInfo<Runtime>;
}

parameter_types! {
    pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
        RuntimeBlockWeights::get().max_block;
}

impl pallet_scheduler::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type PalletsOrigin = OriginCaller;
    type RuntimeCall = RuntimeCall;
    type MaximumWeight = MaximumSchedulerWeight;
    type ScheduleOrigin = EnsureRoot<AccountId>;
    #[cfg(feature = "runtime-benchmarks")]
    type MaxScheduledPerBlock = ConstU32<512>;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type MaxScheduledPerBlock = ConstU32<50>;
    type WeightInfo = pallet_scheduler::weights::SubstrateWeight<Runtime>;
    type OriginPrivilegeCmp = frame_support::traits::EqualPrivilegeOnly;
    type Preimages = Preimage;
}

parameter_types! {
    pub const PreimageBaseDeposit: Balance = CENTS;
    pub const PreimageByteDeposit: Balance = MILLICENTS;
    pub const PreimageHoldReason: RuntimeHoldReason = RuntimeHoldReason::Preimage(pallet_preimage::HoldReason::Preimage);
}

impl pallet_preimage::Config for Runtime {
    type WeightInfo = weights::pallet_preimage::WeightInfo<Runtime>;
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type ManagerOrigin = EnsureRoot<AccountId>;
    type Consideration = frame_support::traits::fungible::HoldConsideration<
        AccountId,
        Balances,
        PreimageHoldReason,
        frame_support::traits::LinearStoragePrice<PreimageBaseDeposit, PreimageByteDeposit, Balance>,
    >;
}

parameter_types! {
    /// 8KB
    pub const MaxBlobSize: u32 = 8192;
    /// 1KB
    pub const MaxIriSize: u32 = 1024;

    /// 128 bytes, for large labels, hash of a label can be used
    pub const MaxAccumulatorLabelSize: u32 = 128;

    pub const MaxAccumulatorParamsSize: u32 = 512;

    /// 128 bytes, for large labels, hash of a label can be used
    pub const MaxOffchainParamsLabelSize: u32 = 128;
    /// 16KB
    pub const MaxOffchainParamsBytesSize: u32 = 65536;

    pub const FixedPublicKeyMaxSize: u32 = 256;
    pub const PSPublicKeyMaxSize: u32 = 65536;

    pub const AccumulatedMaxSize: u32 = 128;

    pub const MaxDidDocRefSize: u16 = 1024;
    pub const MaxDidServiceEndpointIdSize: u16 = 1024;
    pub const MaxDidServiceEndpointOrigins: u16 = 64;
    pub const MaxDidServiceEndpointOriginSize: u16 = 1025;

    pub const MaxPolicyControllers: u32 = 15;
    pub const MinStatusListCredentialSize: u32 = 500;
    pub const MaxStatusListCredentialSize: u32 = 40_000;

    pub const MaxMasterMembers: u32 = 25;
}

impl did::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnDidRemoval = OffchainSignatures;
}

impl revoke::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

impl trusted_entity::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

impl common::Limits for Runtime {
    type MaxPolicyControllers = MaxPolicyControllers;

    type MaxDidDocRefSize = MaxDidDocRefSize;
    type MaxDidServiceEndpointIdSize = MaxDidServiceEndpointIdSize;
    type MaxDidServiceEndpointOriginSize = MaxDidServiceEndpointOriginSize;
    type MaxDidServiceEndpointOrigins = MaxDidServiceEndpointOrigins;

    type MinStatusListCredentialSize = MinStatusListCredentialSize;
    type MaxStatusListCredentialSize = MaxStatusListCredentialSize;

    type MaxPSPublicKeySize = PSPublicKeyMaxSize;
    type MaxBBSPublicKeySize = FixedPublicKeyMaxSize;
    type MaxBBSPlusPublicKeySize = FixedPublicKeyMaxSize;

    type MaxOffchainParamsLabelSize = MaxOffchainParamsLabelSize;
    type MaxOffchainParamsBytesSize = MaxOffchainParamsBytesSize;

    type MaxAccumulatorLabelSize = MaxAccumulatorLabelSize;
    type MaxAccumulatorParamsSize = MaxAccumulatorParamsSize;
    type MaxAccumulatorPublicKeySize = FixedPublicKeyMaxSize;
    type MaxAccumulatorAccumulatedSize = AccumulatedMaxSize;

    type MaxBlobSize = MaxBlobSize;
    type MaxIriSize = MaxIriSize;

    type MaxMasterMembers = MaxMasterMembers;
}

impl status_list_credential::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

impl offchain_signatures::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

impl accumulator::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

impl blob::Config for Runtime {}

impl anchor::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

impl attest::Config for Runtime {}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
    pub enum Runtime {
        // System support stuff.
        System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>} = 0,
        ParachainSystem: cumulus_pallet_parachain_system::{
            Pallet, Call, Config<T>, Storage, Inherent, Event<T>, ValidateUnsigned,
        } = 1,
        InfraParaCore: cumulus_pallet_infra_parachain_core::{Pallet, Call, Storage, Event<T>} = 2,
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent} = 3,
        ParachainInfo: parachain_info::{Pallet, Storage, Config<T>} = 4,

        // Monetary stuff.
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>} = 10,
        TransactionPayment: pallet_transaction_payment::{Pallet, Storage, Event<T>} = 11,
        SystemTokenTxPayment: pallet_system_token_tx_payment::{Pallet, Event<T>} = 12,
        SystemTokenConversion: pallet_system_token_conversion::{Pallet, Event<T>} = 13,

        // Collator support. the order of these 5 are important and shall not change.
        Authorship: pallet_authorship::{Pallet, Storage} = 20,
        CollatorSelection: pallet_collator_selection::{Pallet, Call, Storage, Event<T>, Config<T>} = 21,
        Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>} = 22,
        Aura: pallet_aura::{Pallet, Storage, Config<T>} = 23,
        AuraExt: cumulus_pallet_aura_ext::{Pallet, Storage, Config<T>} = 24,

        // XCM helpers.
        XcmpQueue: cumulus_pallet_xcmp_queue::{Pallet, Call, Storage, Event<T>} = 30,
        InfraXcm: pallet_xcm::{Pallet, Call, Storage, Event<T>, Origin, Config<T>} = 31,
        CumulusXcm: cumulus_pallet_xcm::{Pallet, Event<T>, Origin} = 32,
        DmpQueue: cumulus_pallet_dmp_queue::{Pallet, Call, Storage, Event<T>} = 33,

        // Governance
        Preimage: pallet_preimage::{Pallet, Call, Storage, Event<T>, HoldReason} = 40,
        Scheduler: pallet_scheduler::{Pallet, Call, Storage, Event<T>} = 41,

        // Assets
		Assets: pallet_assets::<Instance1>::{Pallet, Call, Storage, Event<T>, Config<T>} = 50,
		ForeignAssets: pallet_assets::<Instance2>::{Pallet, Call, Storage, Event<T>, Config<T>} = 51,

        // DID.
        DIDModule: did::{Pallet, Call, Storage, Event<T>, Config<T>} = 61,
        Revoke: revoke::{Pallet, Call, Storage, Event} = 62,
        BlobStore: blob::{Pallet, Call, Storage} = 63,
        Anchor: anchor::{Pallet, Call, Storage, Event<T>} = 64,
        Attest: attest::{Pallet, Call, Storage} = 65,
        Accumulator: accumulator::{Pallet, Call, Storage, Event} = 66,
        OffchainSignatures: offchain_signatures::{Pallet, Call, Storage, Event} = 67,
        StatusListCredential: status_list_credential::{Pallet, Call, Storage, Event} = 68,
        TrustedEntity: trusted_entity::{Pallet, Call, Storage, Event} = 69,

        Sudo: pallet_sudo::{Pallet, Call, Storage, Config<T>, Event<T>} = 99,
    }
);

#[cfg(feature = "runtime-benchmarks")]
mod benches {
    frame_benchmarking::define_benchmarks!(
        [frame_system, SystemBench::<Runtime>]
        [pallet_balances, Balances]
        [pallet_session, SessionBench::<Runtime>]
        [pallet_timestamp, Timestamp]
        [pallet_collator_selection, CollatorSelection]
        [cumulus_pallet_xcmp_queue, XcmpQueue]
    );
}

impl_runtime_apis! {
    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration {
            sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
        }

        fn authorities() -> Vec<AuraId> {
            Aura::authorities().into_inner()
        }
    }

    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block)
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Runtime::metadata().into())
        }

        fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
            Runtime::metadata_at_version(version)
        }

        fn metadata_versions() -> sp_std::vec::Vec<u32> {
            Runtime::metadata_versions()
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
        fn account_nonce(account: AccountId) -> Nonce {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
        fn query_info(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }
        fn query_weight_to_fee(weight: Weight) -> Balance {
            TransactionPayment::weight_to_fee(weight)
        }
        fn query_length_to_fee(length: u32) -> Balance {
            TransactionPayment::length_to_fee(length)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<Block, Balance, RuntimeCall>
        for Runtime
    {
        fn query_call_info(
            call: RuntimeCall,
            len: u32,
        ) -> pallet_transaction_payment::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_call_info(call, len)
        }
        fn query_call_fee_details(
            call: RuntimeCall,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_call_fee_details(call, len)
        }
        fn query_weight_to_fee(weight: Weight) -> Balance {
            TransactionPayment::weight_to_fee(weight)
        }
        fn query_length_to_fee(length: u32) -> Balance {
            TransactionPayment::length_to_fee(length)
        }
    }

    impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
        fn collect_collation_info(header: &<Block as BlockT>::Header) -> cumulus_primitives_core::CollationInfo {
            ParachainSystem::collect_collation_info(header)
        }
    }

    #[cfg(feature = "try-runtime")]
    impl frame_try_runtime::TryRuntime<Block> for Runtime {
        fn on_runtime_upgrade(checks: frame_try_runtime::UpgradeCheckSelect) -> (Weight, Weight) {
            let weight = Executive::try_runtime_upgrade(checks).unwrap();
            (weight, RuntimeBlockWeights::get().max_block)
        }

        fn execute_block(
            block: Block,
            state_root_check: bool,
            signature_check: bool,
            select: frame_try_runtime::TryStateSelect,
        ) -> Weight {
            // NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
            // have a backtrace here.
            Executive::try_execute_block(block, state_root_check, signature_check, select).unwrap()
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn benchmark_metadata(extra: bool) -> (
            Vec<frame_benchmarking::BenchmarkList>,
            Vec<frame_support::traits::StorageInfo>,
        ) {
            use frame_benchmarking::{Benchmarking, BenchmarkList};
            use frame_support::traits::StorageInfoTrait;
            use frame_system_benchmarking::Pallet as SystemBench;
            use cumulus_pallet_session_benchmarking::Pallet as SessionBench;

            let mut list = Vec::<BenchmarkList>::new();
            list_benchmarks!(list, extra);

            let storage_info = AllPalletsWithSystem::storage_info();
            return (list, storage_info)
        }

        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            use frame_benchmarking::{Benchmarking, BenchmarkBatch, TrackedStorageKey};

            use frame_system_benchmarking::Pallet as SystemBench;
            impl frame_system_benchmarking::Config for Runtime {}

            use cumulus_pallet_session_benchmarking::Pallet as SessionBench;
            impl cumulus_pallet_session_benchmarking::Config for Runtime {}

            use frame_support::traits::WhitelistedStorageKeys;
            let whitelist = AllPalletsWithSystem::whitelisted_storage_keys();

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);
            add_benchmarks!(params, batches);

            if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
            Ok(batches)
        }
    }
}

cumulus_pallet_parachain_system::register_validate_block! {
    Runtime = Runtime,
    BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
}
