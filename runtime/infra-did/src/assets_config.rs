use super::{
    weights, xcm_config::SelfReserve, AssetManager, Assets, Balances,
    NativeTokenExistentialDeposit, Runtime, RuntimeEvent, RuntimeOrigin,
};

use common_primitives::{
    assets::{
        AssetConfig, AssetIdType, AssetLocation, AssetRegistry, AssetRegistryMetadata,
        AssetStorageMetadata, BalanceType, LocationType, NativeAndNonNative,
    },
    constants::{ASSET_MANAGER_PALLET_ID, INFRADID_DECIMAL},
    types::{AccountId, Balance, CommonAssetId},
};

use frame_support::{
    pallet_prelude::DispatchResult,
    parameter_types,
    traits::{AsEnsureOriginWithArg, ConstU32},
    PalletId,
};
use frame_system::{EnsureRoot, EnsureSigned};
use xcm::VersionedMultiLocation;

parameter_types! {
    pub const AssetDeposit: Balance = 0; // Does not really matter as this will be only called by root
    pub const AssetAccountDeposit: Balance = 0;
    pub const ApprovalDeposit: Balance = 0;
    pub const AssetsStringLimit: u32 = 50;
    pub const MetadataDepositBase: Balance = 0;
    pub const MetadataDepositPerByte: Balance = 0;
}

impl pallet_assets::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = Balance;
    type AssetId = CommonAssetId;
    type Currency = Balances;
    type ForceOrigin = EnsureRoot<AccountId>;
    type AssetDeposit = AssetDeposit;
    type AssetAccountDeposit = AssetAccountDeposit;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type ApprovalDeposit = ApprovalDeposit;
    type StringLimit = AssetsStringLimit;
    type Freezer = ();
    type Extra = ();
    type WeightInfo = weights::pallet_assets::SubstrateWeight<Runtime>;
    type RemoveItemsLimit = ConstU32<1000>;
    type AssetIdParameter = CommonAssetId;
    type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
    type CallbackHandle = ();
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = ();
}

pub struct CommonAssetRegistry;
impl BalanceType for CommonAssetRegistry {
    type Balance = Balance;
}
impl AssetIdType for CommonAssetRegistry {
    type AssetId = CommonAssetId;
}
impl AssetRegistry for CommonAssetRegistry {
    type Metadata = AssetStorageMetadata;
    type Error = sp_runtime::DispatchError;

    fn create_asset(
        asset_id: CommonAssetId,
        metadata: AssetStorageMetadata,
        min_balance: Balance,
        is_sufficient: bool,
    ) -> DispatchResult {
        Assets::force_create(
            RuntimeOrigin::root(),
            asset_id,
            sp_runtime::MultiAddress::Id(AssetManager::account_id()),
            is_sufficient,
            min_balance,
        )?;

        Assets::force_set_metadata(
            RuntimeOrigin::root(),
            asset_id,
            metadata.name,
            metadata.symbol,
            metadata.decimals,
            metadata.is_frozen,
        )?;

        Assets::force_asset_status(
            RuntimeOrigin::root(),
            asset_id,
            AssetManager::account_id().into(),
            AssetManager::account_id().into(),
            AssetManager::account_id().into(),
            AssetManager::account_id().into(),
            min_balance,
            is_sufficient,
            metadata.is_frozen,
        )
    }

    fn update_asset_metadata(
        asset_id: &CommonAssetId,
        metadata: AssetStorageMetadata,
    ) -> DispatchResult {
        Assets::force_set_metadata(
            RuntimeOrigin::root(),
            *asset_id,
            metadata.name,
            metadata.symbol,
            metadata.decimals,
            metadata.is_frozen,
        )
    }
}

parameter_types! {
    pub const StartNonNativeAssetId: CommonAssetId = 8;
    pub const NativeAssetId: CommonAssetId = 1;
    pub NativeAssetLocation: AssetLocation = AssetLocation(
        VersionedMultiLocation::V1(SelfReserve::get()));
    pub NativeAssetMetadata: AssetRegistryMetadata<Balance> = AssetRegistryMetadata {
        metadata: AssetStorageMetadata {
            name: b"InfraDID".to_vec(),
            symbol: b"IDID".to_vec(),
            decimals: INFRADID_DECIMAL,
            is_frozen: false,
        },
        min_balance: NativeTokenExistentialDeposit::get(),
        is_sufficient: true,
    };
    pub const AssetManagerPalletId: PalletId = ASSET_MANAGER_PALLET_ID;
}

pub type InfraDIDConcreteFungibleLedger =
    NativeAndNonNative<Runtime, InfraDIDAssetConfig, Balances, Assets>;

/// AssetConfig implementations for this runtime
#[derive(Clone, Eq, PartialEq)]
pub struct InfraDIDAssetConfig;
impl LocationType for InfraDIDAssetConfig {
    type Location = AssetLocation;
}
impl BalanceType for InfraDIDAssetConfig {
    type Balance = Balance;
}
impl AssetIdType for InfraDIDAssetConfig {
    type AssetId = CommonAssetId;
}
impl AssetConfig<Runtime> for InfraDIDAssetConfig {
    type StartNonNativeAssetId = StartNonNativeAssetId;
    type NativeAssetId = NativeAssetId;
    type AssetRegistryMetadata = AssetRegistryMetadata<Balance>;
    type NativeAssetLocation = NativeAssetLocation;
    type NativeAssetMetadata = NativeAssetMetadata;
    type StorageMetadata = AssetStorageMetadata;
    type AssetRegistry = CommonAssetRegistry;
    type FungibleLedger = InfraDIDConcreteFungibleLedger;
}

impl pallet_asset_manager::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type AssetId = CommonAssetId;
    type Balance = Balance;
    type Location = AssetLocation;
    type AssetConfig = InfraDIDAssetConfig;
    type ModifierOrigin = EnsureRoot<AccountId>;
    type PalletId = AssetManagerPalletId;
    type WeightInfo = weights::pallet_asset_manager::SubstrateWeight<Runtime>;
}
