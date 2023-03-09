//! InfraDID RPC Extensions

use super::*;

/// Instantiate all RPC extensions for infradid.
pub fn create_infradid_full<C, P>(deps: FullDeps<C, P>) -> Result<RpcExtension, sc_service::Error>
where
    C: ProvideRuntimeApi<Block>
        + HeaderBackend<Block>
        + AuxStore
        + HeaderMetadata<Block, Error = BlockChainError>
        + Send
        + Sync
        + 'static,
    C::Api: frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
    C::Api: BlockBuilder<Block>,
    C::Api: core_mods_rpc::CoreModsRuntimeApi<Block, infra_did_runtime::Runtime>,
    P: TransactionPool + Sync + Send + 'static,
{
    use frame_rpc_system::{System, SystemApiServer};
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};

    let mut module = RpcExtension::new(());
    let FullDeps {
        client,
        pool,
        deny_unsafe,
    } = deps;

    module
        .merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())
        .map_err(|e| sc_service::Error::Other(e.to_string()))?;
    module
        .merge(TransactionPayment::new(client.clone()).into_rpc())
        .map_err(|e| sc_service::Error::Other(e.to_string()))?;

    Ok(module)
}
