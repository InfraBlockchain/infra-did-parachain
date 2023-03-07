//! RuntimeApi for client

use common_primitives::types::{AccountId, Balance, Block, Index as Nonce};
use sp_runtime::traits::BlakeTwo256;

/// RuntimeApiCommon + RuntimeApiNimbus: nimbus
///
/// Common RuntimeApi trait bound
pub trait RuntimeApiCommon:
    sp_api::Metadata<Block>
    + sp_api::ApiExt<Block>
    + sp_block_builder::BlockBuilder<Block>
    + sp_offchain::OffchainWorkerApi<Block>
    + sp_session::SessionKeys<Block>
    + sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
    + pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
    + frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>
where
    <Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

/// Extend RuntimeApi trait bound for Nimbus
pub trait RuntimeApiNimbus:
    cumulus_primitives_core::CollectCollationInfo<Block> + nimbus_primitives::NimbusApi<Block>
{
}

impl<Api> RuntimeApiCommon for Api
where
    Api: sp_api::Metadata<Block>
        + sp_api::ApiExt<Block>
        + sp_block_builder::BlockBuilder<Block>
        + sp_offchain::OffchainWorkerApi<Block>
        + sp_session::SessionKeys<Block>
        + sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
        + pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
        + frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
    <Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

impl<Api> RuntimeApiNimbus for Api where
    Api: cumulus_primitives_core::CollectCollationInfo<Block> + nimbus_primitives::NimbusApi<Block>
{
}
