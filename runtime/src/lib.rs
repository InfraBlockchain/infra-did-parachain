#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

#[cfg(not(feature = "std"))]
mod wasm_handlers {
    #[panic_handler]
    #[no_mangle]
    pub fn panic(info: &core::panic::PanicInfo) -> ! {
        unsafe {
            let message = sp_std::alloc::format!("{}", info);
            log::error!("{}", message);
            // logging::log(LogLevel::Error, "runtime", message.as_bytes());
            core::arch::wasm32::unreachable();
        }
    }

    pub fn oom(_: core::alloc::Layout) -> ! {
        log::error!("Runtime memory exhausted. Aborting");
        unsafe {
            core::arch::wasm32::unreachable();
        }
    }
}

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

mod weights;
pub mod xcm_config;

pub use core_mods::{
    accumulator, anchor, attest, bbs_plus, blob, did, keys_and_sigs, master, revoke, trusted_entity,
};

use cumulus_pallet_parachain_system::RelayNumberStrictlyIncreases;
use smallvec::smallvec;
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, Verify},
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, MultiSignature, Percent,
};

use sp_std::{cmp::Ordering, prelude::*};
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use frame_support::{
    construct_runtime,
    dispatch::DispatchClass,
    parameter_types,
    traits::{
        AsEnsureOriginWithArg, ConstBool, ConstU32, ConstU64, ConstU8, EitherOfDiverse, Everything,
        Nothing, PrivilegeCmp,
    },
    weights::{
        constants::WEIGHT_REF_TIME_PER_SECOND, ConstantMultiplier, Weight, WeightToFeeCoefficient,
        WeightToFeeCoefficients, WeightToFeePolynomial,
    },
    PalletId,
};
use frame_system::{
    limits::{BlockLength, BlockWeights},
    EnsureRoot, EnsureSigned,
};
pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
pub use sp_runtime::{MultiAddress, Perbill, Permill};
use xcm_config::{XcmConfig, XcmOriginToTransactDispatchOrigin};

#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

// Polkadot imports
use polkadot_runtime_common::{prod_or_fast, BlockHashCount, SlowAdjustingFeeUpdate};

use weights::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight};

// XCM Imports
use xcm::latest::prelude::BodyId;
use xcm_executor::XcmExecutor;

use pallet_contracts::{
    weights::{SubstrateWeight, WeightInfo},
    DefaultAddressGenerator, Frame, Schedule,
};
pub use pallet_sudo::Call as SudoCall;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// An index to a block.
pub type BlockNumber = u32;

/// The address format for describing accounts.
pub type Address = MultiAddress<AccountId, ()>;

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;

/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;

pub type AssetId = u128;

/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
);

pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
    generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra>;

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;

/// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
/// node's balance type.
///
/// This should typically create a mapping between the following ranges:
///   - `[0, MAXIMUM_BLOCK_WEIGHT]`
///   - `[Balance::min, Balance::max]`
///
/// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
///   - Setting it to `0` will essentially disable the weight fee.
///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
    type Balance = Balance;
    fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
        // in Rococo, extrinsic base weight (smallest non-zero weight) is mapped to 1 MILLIUNIT:
        // in our template, we map to 1/10 of that, or 1/10 MILLIUNIT
        let p = MILLIUNIT / 10;
        let q = 100 * Balance::from(ExtrinsicBaseWeight::get().ref_time());
        smallvec![WeightToFeeCoefficient {
            degree: 1,
            negative: false,
            coeff_frac: Perbill::from_rational(p % q, q),
            coeff_integer: p / q,
        }]
    }
}

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;
    use sp_runtime::{generic, traits::BlakeTwo256};

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;
}

impl_opaque_keys! {
    pub struct SessionKeys {
        pub aura: Aura,
    }
}

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("infra-did"),
    impl_name: create_runtime_str!("infra-did"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 0,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 12000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

// Unit = the base number of indivisible units for balances
pub const UNIT: Balance = 1_000_000_000_000;
pub const MILLIUNIT: Balance = 1_000_000_000;
pub const MICROUNIT: Balance = 1_000_000;

/// The existential deposit. Set to 1/10 of the Connected Relay Chain.
pub const EXISTENTIAL_DEPOSIT: Balance = MILLIUNIT;

const fn deposit(items: u32, bytes: u32) -> Balance {
    (items as Balance * UNIT + (bytes as Balance) * (5 * MILLIUNIT / 100)) / 10
}

/// We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is
/// used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(5);

/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by
/// `Operational` extrinsics.
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

/// We allow for 0.5 of a second of compute with a 12 second average block time.
const MAXIMUM_BLOCK_WEIGHT: Weight = Weight::from_parts(
    WEIGHT_REF_TIME_PER_SECOND.saturating_div(2),
    cumulus_primitives_core::relay_chain::v2::MAX_POV_SIZE as u64,
);

// Prints debug output of the `contracts` pallet to stdout if the node is
// started with `-lruntime::contracts=debug`.
pub const CONTRACTS_DEBUG_OUTPUT: bool = true;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
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
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The aggregated dispatch type that is available for extrinsics.
    type RuntimeCall = RuntimeCall;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = AccountIdLookup<AccountId, ()>;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Index;
    /// The index type for blocks.
    type BlockNumber = BlockNumber;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The header type.
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    /// The ubiquitous origin type.
    type RuntimeOrigin = RuntimeOrigin;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// Runtime version.
    type Version = Version;
    /// Converts a module to an index of this module in the runtime.
    type PalletInfo = PalletInfo;
    /// The data to be stored in an account.
    type AccountData = pallet_balances::AccountData<Balance>;
    /// What to do if a new account is created.
    type OnNewAccount = ();
    /// What to do if an account is fully reaped from the system.
    type OnKilledAccount = ();
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// The basic call filter to use in dispatchable.
    type BaseCallFilter = Everything;
    /// Weight information for the extrinsics of this pallet.
    type SystemWeightInfo = ();
    /// Block & extrinsics weights: base values and limits.
    type BlockWeights = RuntimeBlockWeights;
    /// The maximum length of a block (in bytes).
    type BlockLength = RuntimeBlockLength;
    /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
    type SS58Prefix = SS58Prefix;
    /// The action to take on a Runtime Upgrade
    type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
    pub const MaximumBlockWeight: Weight = MAXIMUM_BLOCK_WEIGHT;
    pub const MaxScheduledPerBlock: u32 = 50;
    pub const NoPreimagePostponement: Option<u32> = Some(10);
}

type ScheduleOrigin = EitherOfDiverse<
    EnsureRoot<AccountId>,
    pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 2>,
>;

type MoreThanHalfCouncil = EitherOfDiverse<
    EnsureRoot<AccountId>,
    pallet_collective::EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>,
>;

/// Used the compare the privilege of an origin inside the scheduler.
pub struct OriginPrivilegeCmp;

impl PrivilegeCmp<OriginCaller> for OriginPrivilegeCmp {
    fn cmp_privilege(left: &OriginCaller, right: &OriginCaller) -> Option<Ordering> {
        if left == right {
            return Some(Ordering::Equal);
        }

        match (left, right) {
            // Root is greater than anything.
            (OriginCaller::system(frame_system::RawOrigin::Root), _) => Some(Ordering::Greater),
            // Check which one has more yes votes.
            (
                OriginCaller::Council(pallet_collective::RawOrigin::Members(l_yes_votes, l_count)),
                OriginCaller::Council(pallet_collective::RawOrigin::Members(r_yes_votes, r_count)),
            ) => Some((l_yes_votes * r_count).cmp(&(r_yes_votes * l_count))),
            // For every other origin we don't care, as they are not used for `ScheduleOrigin`.
            _ => None,
        }
    }
}

impl pallet_scheduler::Config for Runtime {
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeEvent = RuntimeEvent;
    type PalletsOrigin = OriginCaller;
    type RuntimeCall = RuntimeCall;
    type MaximumWeight = MaximumBlockWeight;
    type ScheduleOrigin = ScheduleOrigin;
    type MaxScheduledPerBlock = MaxScheduledPerBlock;
    type WeightInfo = weights::pallet_scheduler::WeightInfo<Runtime>;
    type OriginPrivilegeCmp = OriginPrivilegeCmp;
    type Preimages = Preimage;
}

parameter_types! {
    pub const PreimageMaxSize: u32 = 4096 * 1024;
    pub const PreimageBaseDeposit: Balance = deposit(2, 64);
    pub const PreimageByteDeposit: Balance = deposit(0, 1);
}

impl pallet_preimage::Config for Runtime {
    type WeightInfo = weights::pallet_preimage::WeightInfo<Runtime>;
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type ManagerOrigin = EnsureRoot<AccountId>;
    type BaseDeposit = PreimageBaseDeposit;
    type ByteDeposit = PreimageByteDeposit;
}

impl pallet_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
}

parameter_types! {
    pub const BountyDepositBase: Balance = 1 * UNIT;
    pub const BountyDepositPayoutDelay: BlockNumber = 8 * DAYS;
    pub const BountyUpdatePeriod: BlockNumber = 90 * DAYS;
    pub const MaximumReasonLength: u32 = 16384;
    pub const CuratorDepositMultiplier: Permill = Permill::from_percent(50);
    pub const CuratorDepositMin: Balance = 10 * UNIT;
    pub const CuratorDepositMax: Balance = 200 * UNIT;
    pub const BountyValueMinimum: Balance = 10 * UNIT;
}

impl pallet_bounties::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type BountyDepositBase = BountyDepositBase;
    type BountyDepositPayoutDelay = BountyDepositPayoutDelay;
    type BountyUpdatePeriod = BountyUpdatePeriod;
    type CuratorDepositMultiplier = CuratorDepositMultiplier;
    type CuratorDepositMin = CuratorDepositMin;
    type CuratorDepositMax = CuratorDepositMax;
    type BountyValueMinimum = BountyValueMinimum;
    type ChildBountyManager = ChildBounties;
    type DataDepositPerByte = DataDepositPerByte;
    type MaximumReasonLength = MaximumReasonLength;
    type WeightInfo = weights::pallet_bounties::WeightInfo<Runtime>;
}

parameter_types! {
    pub const MaxActiveChildBountyCount: u32 = 100;
    pub const ChildBountyValueMinimum: Balance = BountyValueMinimum::get() / 10;
}

impl pallet_child_bounties::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxActiveChildBountyCount = MaxActiveChildBountyCount;
    type ChildBountyValueMinimum = ChildBountyValueMinimum;
    type WeightInfo = weights::pallet_child_bounties::WeightInfo<Runtime>;
}

parameter_types! {
    pub const ProposalBond: Permill = Permill::from_percent(5);
    pub const ProposalBondMinimum: Balance = 100 * UNIT;
    pub const ProposalBondMaximum: Balance = 500 * UNIT;
    pub const SpendPeriod: BlockNumber = 24 * DAYS;
    pub const Burn: Permill = Permill::from_percent(1);
    pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");

    pub const TipCountdown: BlockNumber = 1 * DAYS;
    pub const TipFindersFee: Percent = Percent::from_percent(20);
    pub const TipReportDepositBase: Balance = 1 * UNIT;
    pub const DataDepositPerByte: Balance = 1 * MILLIUNIT;
    pub const MaxApprovals: u32 = 100;
    pub const MaxAuthorities: u32 = 100_000;
    pub const MaxKeys: u32 = 10_000;
    pub const MaxPeerInHeartbeats: u32 = 10_000;
    pub const MaxPeerDataEncodingSize: u32 = 1_000;
    pub const RootSpendOriginMaxAmount: Balance = Balance::MAX;
}

type ApproveOrigin = EitherOfDiverse<
    EnsureRoot<AccountId>,
    pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 3, 5>,
>;

impl pallet_treasury::Config for Runtime {
    type PalletId = TreasuryPalletId;
    type Currency = Balances;
    type ApproveOrigin = ApproveOrigin;
    type RejectOrigin = MoreThanHalfCouncil;
    type RuntimeEvent = RuntimeEvent;
    type OnSlash = Treasury;
    type ProposalBond = ProposalBond;
    type ProposalBondMinimum = ProposalBondMinimum;
    type ProposalBondMaximum = ProposalBondMaximum;
    type SpendPeriod = SpendPeriod;
    type Burn = Burn;
    type BurnDestination = ();
    type SpendFunds = Bounties;
    type MaxApprovals = MaxApprovals;
    type WeightInfo = weights::pallet_treasury::WeightInfo<Runtime>;
    type SpendOrigin = frame_system::EnsureRootWithSuccess<AccountId, RootSpendOriginMaxAmount>;
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
    type UncleGenerations = ConstU32<0>;
    type FilterUncle = ();
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
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
    type MaxReserves = ConstU32<50>;
    type ReserveIdentifier = [u8; 8];
}

parameter_types! {
    pub const AssetDeposit: Balance = 10;
    pub const AssetsStringLimit: u32 = 50;
    /// Key = 32 bytes, Value = 36 bytes (32+1+1+1+1)
    // https://github.com/paritytech/substrate/blob/069917b/frame/assets/src/lib.rs#L257L271
    pub const MetadataDepositBase: Balance = deposit(1, 68);
    pub const MetadataDepositPerByte: Balance = deposit(0, 1);
    pub const AssetAccountDeposit: Balance = deposit(1, 18);
}

impl pallet_assets::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = Balance;
    type AssetId = AssetId;
    type Currency = Balances;
    type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
    type ForceOrigin = EnsureRoot<AccountId>;
    type AssetDeposit = AssetDeposit;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type AssetAccountDeposit = AssetAccountDeposit;
    type ApprovalDeposit = ExistentialDeposit;
    type StringLimit = AssetsStringLimit;
    type Freezer = ();
    type Extra = ();
    type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
    type RemoveItemsLimit = ConstU32<1000>;
    type AssetIdParameter = codec::Compact<AssetId>;
    type CallbackHandle = ();
}

parameter_types! {
    /// Relay Chain `TransactionByteFee` / 10
    pub const TransactionByteFee: Balance = 10 * MICROUNIT;
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
    type WeightToFee = WeightToFee;
    type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
    type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
    type OperationalFeeMultiplier = ConstU8<5>;
}

parameter_types! {
    pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
    pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
}

impl cumulus_pallet_parachain_system::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnSystemEvent = ();
    type SelfParaId = parachain_info::Pallet<Runtime>;
    type OutboundXcmpMessageSource = XcmpQueue;
    type DmpMessageHandler = DmpQueue;
    type ReservedDmpWeight = ReservedDmpWeight;
    type XcmpMessageHandler = XcmpQueue;
    type ReservedXcmpWeight = ReservedXcmpWeight;
    type CheckAssociatedRelayNumber = RelayNumberStrictlyIncreases;
}

impl pallet_randomness_collective_flip::Config for Runtime {}

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
}

parameter_types! {
    pub const PotId: PalletId = PalletId(*b"PotStake");
    pub const MaxCandidates: u32 = 1000;
    pub const MinCandidates: u32 = 5;
    pub const SessionLength: BlockNumber = 6 * HOURS;
    pub const MaxInvulnerables: u32 = 100;
    pub const ExecutiveBody: BodyId = BodyId::Executive;
}

// We allow root only to execute privileged collator selection operations.
pub type CollatorSelectionUpdateOrigin = EnsureRoot<AccountId>;

impl pallet_collator_selection::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type UpdateOrigin = CollatorSelectionUpdateOrigin;
    type PotId = PotId;
    type MaxCandidates = MaxCandidates;
    type MinCandidates = MinCandidates;
    type MaxInvulnerables = MaxInvulnerables;
    // should be a multiple of session or things will get inconsistent
    type KickThreshold = Period;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
    type ValidatorRegistration = Session;
    type WeightInfo = ();
}

parameter_types! {
    pub const DepositPerItem: Balance = deposit(1, 0);
    pub const DepositPerByte: Balance = deposit(0, 1);
    // The lazy deletion runs inside on_initialize.
    pub DeletionWeightLimit: Weight = AVERAGE_ON_INITIALIZE_RATIO *
        RuntimeBlockWeights::get().max_block;
    // The weight needed for decoding the queue should be less or equal than a fifth
    // of the overall weight dedicated to the lazy deletion.
    pub DeletionQueueDepth: u32 = ((DeletionWeightLimit::get().ref_time() / (
            <Runtime as pallet_contracts::Config>::WeightInfo::on_initialize_per_queue_item(1).ref_time() -
            <Runtime as pallet_contracts::Config>::WeightInfo::on_initialize_per_queue_item(0).ref_time()
        )) / 5) as u32;
    pub MySchedule: Schedule<Runtime> = Default::default();
}

impl pallet_contracts::Config for Runtime {
    type Time = Timestamp;
    type Randomness = RandomnessCollectiveFlip;
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    /// The safest default is to allow no calls at all.
    ///
    /// Runtimes should whitelist dispatchables that are allowed to be called from contracts
    /// and make sure they are stable. Dispatchables exposed to contracts are not allowed to
    /// change because that would break already deployed contracts. The `Call` structure itself
    /// is not allowed to change the indices of existing pallets, too.
    type CallFilter = Nothing;
    type DepositPerItem = DepositPerItem;
    type DepositPerByte = DepositPerByte;
    type WeightPrice = pallet_transaction_payment::Pallet<Self>;
    type WeightInfo = SubstrateWeight<Self>;
    type ChainExtension = ();
    type DeletionQueueDepth = DeletionQueueDepth;
    type DeletionWeightLimit = DeletionWeightLimit;
    type Schedule = MySchedule;
    type CallStack = [Frame<Self>; 31];
    type AddressGenerator = DefaultAddressGenerator;
    type MaxCodeLen = ConstU32<{ 128 * 1024 }>;
    type MaxStorageKeyLen = ConstU32<128>;
    type UnsafeUnstableInterface = ConstBool<true>;
    type MaxDebugBufferLen = ConstU32<{ 2 * 1024 * 1024 }>;
}

parameter_types! {
    pub LaunchPeriod: BlockNumber = prod_or_fast!(28 * DAYS, 1, "DOT_LAUNCH_PERIOD");
    pub VotingPeriod: BlockNumber = prod_or_fast!(28 * DAYS, 1 * MINUTES, "DOT_VOTING_PERIOD");
    pub FastTrackVotingPeriod: BlockNumber = prod_or_fast!(3 * HOURS, 1 * MINUTES, "DOT_FAST_TRACK_VOTING_PERIOD");
    pub const MinimumDeposit: Balance = 100 * UNIT;
    pub EnactmentPeriod: BlockNumber = prod_or_fast!(28 * DAYS, 1, "DOT_ENACTMENT_PERIOD");
    pub CooloffPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1, "DOT_COOLOFF_PERIOD");
    pub const InstantAllowed: bool = true;
    pub const MaxVotes: u32 = 100;
    pub const MaxProposals: u32 = 100;
}

impl pallet_democracy::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type EnactmentPeriod = EnactmentPeriod;
    type VoteLockingPeriod = EnactmentPeriod;
    type LaunchPeriod = LaunchPeriod;
    type VotingPeriod = VotingPeriod;
    type MinimumDeposit = MinimumDeposit;
    /// A straight majority of the council can decide what their next motion is.
    type ExternalOrigin = EitherOfDiverse<
        pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 2>,
        frame_system::EnsureRoot<AccountId>,
    >;
    /// A 60% super-majority can have the next scheduled referendum be a straight majority-carries vote.
    type ExternalMajorityOrigin = EitherOfDiverse<
        pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 3, 5>,
        frame_system::EnsureRoot<AccountId>,
    >;
    /// A unanimous council can have the next scheduled referendum be a straight default-carries
    /// (NTB) vote.
    type ExternalDefaultOrigin = EitherOfDiverse<
        pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 1>,
        frame_system::EnsureRoot<AccountId>,
    >;
    /// Two thirds of the technical committee can have an `ExternalMajority/ExternalDefault` vote
    /// be tabled immediately and with a shorter voting/enactment period.
    type FastTrackOrigin = EitherOfDiverse<
        pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 2, 3>,
        frame_system::EnsureRoot<AccountId>,
    >;
    type InstantOrigin = EitherOfDiverse<
        pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 1, 1>,
        frame_system::EnsureRoot<AccountId>,
    >;
    type InstantAllowed = InstantAllowed;
    type FastTrackVotingPeriod = FastTrackVotingPeriod;
    // To cancel a proposal which has been passed, 2/3 of the council must agree to it.
    type CancellationOrigin = EitherOfDiverse<
        pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 2, 3>,
        EnsureRoot<AccountId>,
    >;
    // To cancel a proposal before it has been passed, the technical committee must be unanimous or
    // Root must agree.
    type CancelProposalOrigin = EitherOfDiverse<
        pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 1, 1>,
        EnsureRoot<AccountId>,
    >;
    type BlacklistOrigin = EnsureRoot<AccountId>;
    // Any single technical committee member may veto a coming council proposal, however they can
    // only do it once and it lasts only for the cooloff period.
    type VetoOrigin = pallet_collective::EnsureMember<AccountId, TechnicalCollective>;
    type CooloffPeriod = CooloffPeriod;
    type Slash = Treasury;
    type Scheduler = Scheduler;
    type PalletsOrigin = OriginCaller;
    type MaxVotes = MaxVotes;
    type WeightInfo = weights::pallet_democracy::WeightInfo<Runtime>;
    type MaxProposals = MaxProposals;
    type Preimages = Preimage;
    type MaxDeposits = ConstU32<100>;
    type MaxBlacklisted = ConstU32<100>;
}

parameter_types! {
    pub CouncilMotionDuration: BlockNumber = prod_or_fast!(7 * DAYS, 2 * MINUTES, "DOT_MOTION_DURATION");
    pub const CouncilMaxProposals: u32 = 100;
    pub const CouncilMaxMembers: u32 = 100;
}

pub type CouncilCollective = pallet_collective::Instance1;
impl pallet_collective::Config<CouncilCollective> for Runtime {
    type RuntimeOrigin = RuntimeOrigin;
    type Proposal = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type MotionDuration = CouncilMotionDuration;
    type MaxProposals = CouncilMaxProposals;
    type MaxMembers = CouncilMaxMembers;
    type DefaultVote = pallet_collective::PrimeDefaultVote;
    type WeightInfo = weights::pallet_collective_council::WeightInfo<Runtime>;
}

parameter_types! {
    pub const TechnicalMotionDuration: BlockNumber = 7 * DAYS;
    pub const TechnicalMaxProposals: u32 = 100;
    pub const TechnicalMaxMembers: u32 = 100;
}

pub type TechnicalCollective = pallet_collective::Instance2;
impl pallet_collective::Config<TechnicalCollective> for Runtime {
    type RuntimeOrigin = RuntimeOrigin;
    type Proposal = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type MotionDuration = TechnicalMotionDuration;
    type MaxProposals = TechnicalMaxProposals;
    type MaxMembers = TechnicalMaxMembers;
    type DefaultVote = pallet_collective::PrimeDefaultVote;
    type WeightInfo = weights::pallet_collective_technical_committee::WeightInfo<Runtime>;
}

parameter_types! {
    pub const UnsignedPriority: u64 = 1 << 20;
}

parameter_types! {
    // 8KB
    pub const MaxBlobSize: u32 = 8192;
    pub const StorageWeight: Weight = Weight::from_ref_time(1100);
    // 128 bytes, for large labels, hash of a label can be used
    pub const LabelMaxSize: u32 = 128;
    pub const LabelPerByteWeight: Weight = Weight::from_ref_time(10);
    // 16KB
    pub const ParamsMaxSize: u32 = 65536;
    pub const ParamsPerByteWeight: Weight = Weight::from_ref_time(10);
    pub const PublicKeyMaxSize: u32 = 256;
    pub const PublicKeyPerByteWeight: Weight = Weight::from_ref_time(10);
    pub const AccumulatorParamsMaxSize: u32 = 512;
    pub const AccumulatorParamsPerByteWeight: Weight = Weight::from_ref_time(10);
    pub const AccumulatedMaxSize: u32 = 128;
    pub const AccumulatedPerByteWeight: Weight = Weight::from_ref_time(10);
    pub const MaxDidDocRefSize: u16 = 1024;
    pub const DidDocRefPerByteWeight: Weight = Weight::from_ref_time(10);
    pub const MaxServiceEndpointIdSize: u16 = 1024;
    pub const ServiceEndpointIdPerByteWeight: Weight = Weight::from_ref_time(10);
    pub const MaxServiceEndpointOrigins: u16 = 64;
    pub const MaxServiceEndpointOriginSize: u16 = 1025;
    pub const ServiceEndpointOriginPerByteWeight: Weight = Weight::from_ref_time(10);
    pub const MaxControllers: u32 = 15;
}

impl did::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxDidDocRefSize = MaxDidDocRefSize;
    type DidDocRefPerByteWeight = DidDocRefPerByteWeight;
    type MaxServiceEndpointIdSize = MaxServiceEndpointIdSize;
    type ServiceEndpointIdPerByteWeight = ServiceEndpointIdPerByteWeight;
    type MaxServiceEndpointOrigins = MaxServiceEndpointOrigins;
    type MaxServiceEndpointOriginSize = MaxServiceEndpointOriginSize;
    type ServiceEndpointOriginPerByteWeight = ServiceEndpointOriginPerByteWeight;
}

impl revoke::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxControllers = MaxControllers;
}

impl bbs_plus::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type LabelMaxSize = LabelMaxSize;
    type LabelPerByteWeight = LabelPerByteWeight;
    type ParamsMaxSize = ParamsMaxSize;
    type ParamsPerByteWeight = ParamsPerByteWeight;
    type PublicKeyMaxSize = PublicKeyMaxSize;
    type PublicKeyPerByteWeight = PublicKeyPerByteWeight;
}

impl accumulator::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type LabelMaxSize = LabelMaxSize;
    type LabelPerByteWeight = LabelPerByteWeight;
    type ParamsMaxSize = AccumulatorParamsMaxSize;
    type ParamsPerByteWeight = AccumulatorParamsPerByteWeight;
    type PublicKeyMaxSize = PublicKeyMaxSize;
    type PublicKeyPerByteWeight = PublicKeyPerByteWeight;
    type AccumulatedMaxSize = AccumulatedMaxSize;
    type AccumulatedPerByteWeight = AccumulatedPerByteWeight;
}

impl blob::Config for Runtime {
    type MaxBlobSize = MaxBlobSize;
    type StorageWeight = StorageWeight;
}

impl master::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
}

impl anchor::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

impl attest::Config for Runtime {
    type StorageWeight = StorageWeight;
}

impl trusted_entity::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxControllers = MaxControllers;
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        // System support stuff.
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>} = 0,
        ParachainSystem: cumulus_pallet_parachain_system::{
            Pallet, Call, Config, Storage, Inherent, Event<T>, ValidateUnsigned,
        } = 1,
        RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage} = 2,
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent} = 3,
        ParachainInfo: parachain_info::{Pallet, Storage, Config} = 4,
        Scheduler: pallet_scheduler::{Pallet, Call, Storage, Event<T>} = 5,
        Preimage: pallet_preimage::{Pallet, Call, Storage, Event<T>} = 6,

        // Monetary stuff.
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>} = 10,
        TransactionPayment: pallet_transaction_payment::{Pallet, Storage, Event<T>} = 11,
        Assets: pallet_assets::{Pallet, Call, Storage, Config<T>, Event<T>} = 12,

        // Governance stuff.
        Democracy: pallet_democracy::{Pallet, Call, Storage, Config<T>, Event<T>} = 14,
        Council: pallet_collective::<Instance1>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>} = 15,
        TechnicalCommittee: pallet_collective::<Instance2>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>} = 16,
        Treasury: pallet_treasury::{Pallet, Call, Storage, Config, Event<T>} = 19,

        // Collator support. The order of these 4 are important and shall not change.
        Authorship: pallet_authorship::{Pallet, Call, Storage} = 20,
        CollatorSelection: pallet_collator_selection::{Pallet, Call, Storage, Event<T>, Config<T>} = 21,
        Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>} = 22,
        Aura: pallet_aura::{Pallet, Storage, Config<T>} = 23,
        AuraExt: cumulus_pallet_aura_ext::{Pallet, Storage, Config} = 24,

        // XCM helpers.
        XcmpQueue: cumulus_pallet_xcmp_queue::{Pallet, Call, Storage, Event<T>} = 30,
        PolkadotXcm: pallet_xcm::{Pallet, Call, Event<T>, Origin, Config} = 31,
        CumulusXcm: cumulus_pallet_xcm::{Pallet, Event<T>, Origin} = 32,
        DmpQueue: cumulus_pallet_dmp_queue::{Pallet, Call, Storage, Event<T>} = 33,

        // Bounties modules.
        Bounties: pallet_bounties::{Pallet, Call, Storage, Event<T>} = 34,
        ChildBounties: pallet_child_bounties = 38,

        // Smart Contracts.
        Contracts: pallet_contracts::{Pallet, Call, Storage, Event<T>} = 40,

        // DID
        BbsPlus: bbs_plus::{Pallet, Call, Storage, Event} = 51,
        DIDModule: did::{Pallet, Call, Storage, Event, Config} = 52,
        Revoke: revoke::{Pallet, Call, Storage, Event} = 53,
        BlobStore: blob::{Pallet, Call, Storage} = 54,
        Master: master::{Pallet, Call, Storage, Event<T>, Config} = 55,
        Anchor: anchor::{Pallet, Call, Storage, Event<T>} = 56,
        Attest: attest::{Pallet, Call, Storage} = 57,
        Accumulator: accumulator::{Pallet, Call, Storage, Event} = 58,
        TrustedEntity: trusted_entity::{Pallet, Call, Storage, Event} = 59,

        Sudo: pallet_sudo::{Pallet, Call, Storage, Config<T>, Event<T>} = 100,
    }
);

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
    define_benchmarks!(
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

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
        fn account_nonce(account: AccountId) -> Index {
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
    }

    impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
        fn collect_collation_info(header: &<Block as BlockT>::Header) -> cumulus_primitives_core::CollationInfo {
            ParachainSystem::collect_collation_info(header)
        }
    }

    impl pallet_contracts::ContractsApi<Block, AccountId, Balance, BlockNumber, Hash> for Runtime {
        fn call(
            origin: AccountId,
            dest: AccountId,
            value: Balance,
            gas_limit: Option<Weight>,
            storage_deposit_limit: Option<Balance>,
            input_data: Vec<u8>,
        ) -> pallet_contracts_primitives::ContractExecResult<Balance> {
            let gas_limit = gas_limit.unwrap_or(RuntimeBlockWeights::get().max_block);
            Contracts::bare_call(
                origin,
                dest,
                value,
                gas_limit,
                storage_deposit_limit,
                input_data,
                CONTRACTS_DEBUG_OUTPUT,
                pallet_contracts::Determinism::Deterministic,
            )
        }

        fn instantiate(
            origin: AccountId,
            value: Balance,
            gas_limit: Option<Weight>,
            storage_deposit_limit: Option<Balance>,
            code: pallet_contracts_primitives::Code<Hash>,
            data: Vec<u8>,
            salt: Vec<u8>,
        ) -> pallet_contracts_primitives::ContractInstantiateResult<AccountId, Balance> {
            let gas_limit = gas_limit.unwrap_or(RuntimeBlockWeights::get().max_block);
            Contracts::bare_instantiate(
                origin,
                value,
                gas_limit,
                storage_deposit_limit,
                code,
                data,
                salt,
                CONTRACTS_DEBUG_OUTPUT,
            )
        }

        fn upload_code(
            origin: AccountId,
            code: Vec<u8>,
            storage_deposit_limit: Option<Balance>,
            determinism: pallet_contracts::Determinism,
        ) -> pallet_contracts_primitives::CodeUploadResult<Hash, Balance> {
            Contracts::bare_upload_code(
                origin,
                code,
                storage_deposit_limit,
                determinism,
            )
        }

        fn get_storage(
            address: AccountId,
            key: Vec<u8>,
        ) -> pallet_contracts_primitives::GetStorageResult {
            Contracts::get_storage(address, key)
        }
    }

    #[cfg(feature = "try-runtime")]
    impl frame_try_runtime::TryRuntime<Block> for Runtime {
        fn on_runtime_upgrade(checks: bool) -> (Weight, Weight) {
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

            let whitelist: Vec<TrackedStorageKey> = vec![
                // Block Number
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
                // Total Issuance
                hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
                // Execution Phase
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
                // Event Count
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
                // System Events
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
            ];

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);
            add_benchmarks!(params, batches);

            if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
            Ok(batches)
        }
    }
}

struct CheckInherents;

impl cumulus_pallet_parachain_system::CheckInherents<Block> for CheckInherents {
    fn check_inherents(
        block: &Block,
        relay_state_proof: &cumulus_pallet_parachain_system::RelayChainStateProof,
    ) -> sp_inherents::CheckInherentsResult {
        let relay_chain_slot = relay_state_proof
            .read_slot()
            .expect("Could not read the relay chain slot from the proof");

        let inherent_data =
            cumulus_primitives_timestamp::InherentDataProvider::from_relay_chain_slot_and_duration(
                relay_chain_slot,
                sp_std::time::Duration::from_secs(6),
            )
            .create_inherent_data()
            .expect("Could not create the timestamp inherent data");

        inherent_data.check_extrinsics(block)
    }
}

cumulus_pallet_parachain_system::register_validate_block! {
    Runtime = Runtime,
    BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
    CheckInherents = CheckInherents,
}
