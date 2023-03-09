//! Chain-Specific Command Line Interfaces

use crate::{
    chain_specs,
    cli::{Cli, RelayChainCli, Subcommand},
    rpc,
    service::{new_partial, InfraDIDRuntimeExecutor},
};
use codec::Encode;
use common_primitives::types::Header;
use cumulus_client_cli::generate_genesis_block;
use cumulus_primitives_core::ParaId;
use frame_benchmarking_cli::{BenchmarkCmd, SUBSTRATE_REFERENCE_HARDWARE};
use log::info;
use sc_cli::{
    ChainSpec, CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams,
    NetworkParams, RuntimeVersion, SharedParams, SubstrateCli,
};
use sc_service::config::{BasePath, PrometheusConfig};
use sp_core::hexdisplay::HexDisplay;
use sp_runtime::{
    generic,
    traits::{AccountIdConversion, Block as BlockT},
    OpaqueExtrinsic,
};
use std::net::SocketAddr;

pub use sc_cli::Error;

/// Result Type Alias with default [`Error`] Type
pub type Result<T = (), E = Error> = core::result::Result<T, E>;

/// Block Type
pub type Block = generic::Block<Header, OpaqueExtrinsic>;

/// Parachain ID
pub const INFRADID_PARACHAIN_ID: u32 = 1337;

trait IdentifyChain {
    fn is_infradid(&self) -> bool;
    fn is_localdev(&self) -> bool;
}

impl IdentifyChain for dyn sc_service::ChainSpec {
    fn is_infradid(&self) -> bool {
        self.id().starts_with("infradid")
    }
    fn is_localdev(&self) -> bool {
        self.id().ends_with("localdev")
    }
}

impl<T: sc_service::ChainSpec + 'static> IdentifyChain for T {
    fn is_infradid(&self) -> bool {
        <dyn sc_service::ChainSpec>::is_infradid(self)
    }
    fn is_localdev(&self) -> bool {
        <dyn sc_service::ChainSpec>::is_localdev(self)
    }
}

fn load_spec(id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
    match id {
        // infradid chainspec
        "infradid-dev" => Ok(Box::new(chain_specs::infradid_development_config())),
        "infradid-local" => Ok(Box::new(chain_specs::infradid_local_config(false))),
        "infradid-localdev" => Ok(Box::new(chain_specs::infradid_local_config(true))),
        // "infradid-testnet" => Ok(Box::new(chain_specs::infradid_testnet_config()?)),
        // "infradid-2085" => Ok(Box::new(chain_specs::infradid_2085_config()?)),
        // "infradid-v3-staging" => Ok(Box::new(chain_specs::infradid_v3_2085_staging_config()?)),
        path => {
            let chain_spec = chain_specs::ChainSpec::from_json_file(path.into())?;
            if chain_spec.is_infradid() {
                Ok(Box::new(chain_specs::InfraDIDChainSpec::from_json_file(
                    path.into(),
                )?))
            } else {
                Err("Please input a vaild file name.".into())
            }
        }
    }
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Parachain Collator".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        format!(
            "Parachain Collator\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relaychain node.\n\n\
		{} [parachain-args] -- [relaychain-args]",
            Self::executable_name()
        )
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/sweatpotato13/substrate-parachain-boilerplate/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2023
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
        load_spec(id)
    }

    fn native_runtime_version(chain_spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        if chain_spec.is_infradid() {
            &infra_did_runtime::VERSION
        } else {
            panic!("invalid chain spec!")
        }
    }
}

impl SubstrateCli for RelayChainCli {
    fn impl_name() -> String {
        "Infra-DID Parachain Collator".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        format!(
            "Infra-DID Parachain collator\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relaychain node.\n\n\
		{} [parachain-args] -- [relaychain-args]",
            Self::executable_name()
        )
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/sweatpotato13/substrate-parachain-boilerplate/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2023
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
        polkadot_cli::Cli::from_iter([RelayChainCli::executable_name()].iter()).load_spec(id)
    }

    fn native_runtime_version(chain_spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        polkadot_cli::Cli::native_runtime_version(chain_spec)
    }
}

/// Creates partial components for the runtimes that are supported by the benchmarks.
macro_rules! construct_benchmark_partials {
    ($config:expr, |$partials:ident| $code:expr) => {
        if $config.chain_spec.is_infradid() {
            let $partials = new_partial::<infra_did_runtime::RuntimeApi>(&$config)?;
            $code
        } else {
            Err("The chain is not supported".into())
        }
    };
}

macro_rules! construct_async_run {
    (|$components:ident, $cli:ident, $cmd:ident, $config:ident| $( $code:tt )* ) => {{
        let runner = $cli.create_runner($cmd)?;
            if runner.config().chain_spec.is_infradid() {
                runner.async_run(|$config| {
                    let $components = new_partial::<infra_did_runtime::RuntimeApi>(
                        &$config,
                    )?;
                    let task_manager = $components.task_manager;
                    { $( $code )* }.map(|v| (v, task_manager))
                })
            } else {
                panic!("wrong chain spec");
            }
    }}
}

/// Parse command line arguments into service configuration.
pub fn run_with(cli: Cli) -> Result {
    match &cli.subcommand {
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, components.import_queue))
            })
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, config.database))
            })
        }
        Some(Subcommand::ExportState(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, config.chain_spec))
            })
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, components.import_queue))
            })
        }
        Some(Subcommand::Revert(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, components.backend, None))
            })
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            runner.sync_run(|config| {
                let polkadot_cli = RelayChainCli::new(
                    &config,
                    [RelayChainCli::executable_name()]
                        .iter()
                        .chain(cli.relaychain_args.iter()),
                );

                let polkadot_config = SubstrateCli::create_configuration(
                    &polkadot_cli,
                    &polkadot_cli,
                    config.tokio_handle.clone(),
                )
                .map_err(|err| format!("Relay chain argument error: {err}"))?;

                cmd.run(config, polkadot_config)
            })
        }
        Some(Subcommand::ExportGenesisState(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|_config| {
                let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
                let state_version = Cli::native_runtime_version(&spec).state_version();
                cmd.run::<Block>(&*spec, state_version)
            })
        }
        Some(Subcommand::ExportGenesisWasm(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|_config| {
                let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
                cmd.run(&*spec)
            })
        }
        Some(Subcommand::Benchmark(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            // Switch on the concrete benchmark sub-command
            match cmd {
                BenchmarkCmd::Pallet(cmd) => {
                    if cfg!(feature = "runtime-benchmarks") {
                        runner.sync_run(|config| {
                            if config.chain_spec.is_infradid() {
                                cmd.run::<Block, InfraDIDRuntimeExecutor>(config)
                            } else {
                                Err("Chain doesn't support benchmarking".into())
                            }
                        })
                    } else {
                        Err("Benchmarking wasn't enabled when building the node. \
				You can enable it with `--features runtime-benchmarks`."
                            .into())
                    }
                }
                BenchmarkCmd::Block(cmd) => runner.sync_run(|config| {
                    construct_benchmark_partials!(config, |partials| cmd.run(partials.client))
                }),
                #[cfg(not(feature = "runtime-benchmarks"))]
                BenchmarkCmd::Storage(_) => Err(
                    "Storage benchmarking can be enabled with `--features runtime-benchmarks`."
                        .into(),
                ),
                #[cfg(feature = "runtime-benchmarks")]
                BenchmarkCmd::Storage(cmd) => runner.sync_run(|config| {
                    construct_benchmark_partials!(config, |partials| {
                        let db = partials.backend.expose_db();
                        let storage = partials.backend.expose_storage();

                        cmd.run(config, partials.client.clone(), db, storage)
                    })
                }),
                BenchmarkCmd::Extrinsic(_) => Err("Unsupported benchmarking command".into()),
                BenchmarkCmd::Overhead(_) => Err("Unsupported benchmarking command".into()),
                BenchmarkCmd::Machine(cmd) => {
                    runner.sync_run(|config| cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone()))
                }
            }
        }
        #[cfg(feature = "try-runtime")]
        Some(Subcommand::TryRuntime(cmd)) => {
            // grab the task manager.
            let runner = cli.create_runner(cmd)?;
            let registry = &runner
                .config()
                .prometheus_config
                .as_ref()
                .map(|cfg| &cfg.registry);
            let task_manager =
                sc_service::TaskManager::new(runner.config().tokio_handle.clone(), *registry)
                    .map_err(|e| format!("Error: {e:?}"))?;

            if runner.config().chain_spec.is_infradid() {
                runner.async_run(|config| {
                    Ok((
                        cmd.run::<Block, InfraDIDRuntimeExecutor>(config),
                        task_manager,
                    ))
                })
            } else {
                Err("Chain doesn't support try-runtime".into())
            }
        }
        #[cfg(not(feature = "try-runtime"))]
        Some(Subcommand::TryRuntime) => Err("Try-runtime wasn't enabled when building the node. \
		You can enable it with `--features try-runtime`."
            .into()),
        None => {
            let runner = cli.create_runner(&cli.run.normalize())?;
            let chain_spec = &runner.config().chain_spec;
            let is_dev = chain_spec.is_localdev();
            info!("id:{}", chain_spec.id());
            let collator_options = cli.run.collator_options();

            runner.run_node_until_exit(|config| async move {
                if is_dev {
                    info!("⚠️  DEV STANDALONE MODE.");
                    if config.chain_spec.is_infradid() {
                        return crate::service::start_dev_nimbus_node::<infra_did_runtime::RuntimeApi, _>(
                            config,
                            rpc::create_infradid_full,
                        ).await
                            .map_err(Into::into);
                    } else {
                        return Err("Dev mode not support for current chain".into());
                    }
                }

                let hwbench = if !cli.no_hardware_benchmarks {
                    config.database.path().map(|database_path| {
                        let _ = std::fs::create_dir_all(database_path);
                        sc_sysinfo::gather_hwbench(Some(database_path))
                    })
                } else {
                    None
                };

                let para_id = crate::chain_specs::Extensions::try_get(&*config.chain_spec)
                    .map(|e| e.para_id)
                    .ok_or("Could not find parachain extension in chain-spec.")?;

                let polkadot_cli = RelayChainCli::new(
                    &config,
                    [RelayChainCli::executable_name()]
                        .iter()
                        .chain(cli.relaychain_args.iter()),
                );

                let id = ParaId::from(para_id);

                let parachain_account =
                    AccountIdConversion::<polkadot_primitives::v2::AccountId>::into_account_truncating(&id);

                let state_version =
                    RelayChainCli::native_runtime_version(&config.chain_spec).state_version();

                let block: crate::service::Block =
                    generate_genesis_block(&*config.chain_spec, state_version)
                        .map_err(|e| format!("{e:?}"))?;
                let genesis_state = format!("0x{:?}", HexDisplay::from(&block.header().encode()));

                let tokio_handle = config.tokio_handle.clone();
                let polkadot_config =
                    SubstrateCli::create_configuration(&polkadot_cli, &polkadot_cli, tokio_handle)
                        .map_err(|err| format!("Relay chain argument error: {err}"))?;

                info!("Parachain id: {:?}", id);
                info!("Parachain Account: {}", parachain_account);
                info!("Parachain genesis state: {}", genesis_state);
                info!(
                    "Is collating: {}",
                    if config.role.is_authority() {
                        "yes"
                    } else {
                        "no"
                    }
                );

                if config.chain_spec.is_infradid() {
                    crate::service::start_parachain_node::<infra_did_runtime::RuntimeApi, _>(
                        config,
                        polkadot_config,
                        collator_options,
                        id,
                        hwbench,
                        rpc::create_infradid_full,
                    )
                    .await
                    .map(|r| r.0)
                    .map_err(Into::into)
                } else {
                    Err("chain spec error".into())
                }
            })
        }
    }
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result {
    run_with(Cli::from_args())
}

impl DefaultConfigurationValues for RelayChainCli {
    fn p2p_listen_port() -> u16 {
        30334
    }

    fn rpc_ws_listen_port() -> u16 {
        9945
    }

    fn rpc_http_listen_port() -> u16 {
        9934
    }

    fn prometheus_listen_port() -> u16 {
        9616
    }
}

impl CliConfiguration<Self> for RelayChainCli {
    fn shared_params(&self) -> &SharedParams {
        self.base.base.shared_params()
    }

    fn import_params(&self) -> Option<&ImportParams> {
        self.base.base.import_params()
    }

    fn network_params(&self) -> Option<&NetworkParams> {
        self.base.base.network_params()
    }

    fn keystore_params(&self) -> Option<&KeystoreParams> {
        self.base.base.keystore_params()
    }

    fn base_path(&self) -> Result<Option<BasePath>> {
        Ok(self
            .shared_params()
            .base_path()?
            .or_else(|| self.base_path.clone().map(Into::into)))
    }

    fn rpc_http(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
        self.base.base.rpc_http(default_listen_port)
    }

    fn rpc_ipc(&self) -> Result<Option<String>> {
        self.base.base.rpc_ipc()
    }

    fn rpc_ws(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
        self.base.base.rpc_ws(default_listen_port)
    }

    fn prometheus_config(
        &self,
        default_listen_port: u16,
        chain_spec: &Box<dyn ChainSpec>,
    ) -> Result<Option<PrometheusConfig>> {
        self.base
            .base
            .prometheus_config(default_listen_port, chain_spec)
    }

    fn init<F>(
        &self,
        _support_url: &String,
        _impl_version: &String,
        _logger_hook: F,
        _config: &sc_service::Configuration,
    ) -> Result<()>
    where
        F: FnOnce(&mut sc_cli::LoggerBuilder, &sc_service::Configuration),
    {
        unreachable!("PolkadotCli is never initialized; qed");
    }

    fn chain_id(&self, is_dev: bool) -> Result<String> {
        let chain_id = self.base.base.chain_id(is_dev)?;

        Ok(if chain_id.is_empty() {
            self.chain_id.clone().unwrap_or_default()
        } else {
            chain_id
        })
    }

    fn role(&self, is_dev: bool) -> Result<sc_service::Role> {
        self.base.base.role(is_dev)
    }

    fn transaction_pool(&self, is_dev: bool) -> Result<sc_service::config::TransactionPoolOptions> {
        self.base.base.transaction_pool(is_dev)
    }

    fn rpc_methods(&self) -> Result<sc_service::config::RpcMethods> {
        self.base.base.rpc_methods()
    }

    fn rpc_ws_max_connections(&self) -> Result<Option<usize>> {
        self.base.base.rpc_ws_max_connections()
    }

    fn rpc_cors(&self, is_dev: bool) -> Result<Option<Vec<String>>> {
        self.base.base.rpc_cors(is_dev)
    }

    fn default_heap_pages(&self) -> Result<Option<u64>> {
        self.base.base.default_heap_pages()
    }

    fn force_authoring(&self) -> Result<bool> {
        self.base.base.force_authoring()
    }

    fn disable_grandpa(&self) -> Result<bool> {
        self.base.base.disable_grandpa()
    }

    fn max_runtime_instances(&self) -> Result<Option<usize>> {
        self.base.base.max_runtime_instances()
    }

    fn announce_block(&self) -> Result<bool> {
        self.base.base.announce_block()
    }

    fn telemetry_endpoints(
        &self,
        chain_spec: &Box<dyn ChainSpec>,
    ) -> Result<Option<sc_telemetry::TelemetryEndpoints>> {
        self.base.base.telemetry_endpoints(chain_spec)
    }
}