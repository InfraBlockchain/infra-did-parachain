//! Parachain Chain Specifications

use super::*;
use crate::command::INFRADID_PARACHAIN_ID;
pub use infra_did_runtime::{
    did::{Did, DidKey},
    keys_and_sigs::PublicKey,
    master::Membership,
    opaque::SessionKeys,
    CouncilConfig, DIDModuleConfig, DemocracyConfig, GenesisConfig, MasterConfig,
    TechnicalCommitteeConfig,
};
use session_key_primitives::util::unchecked_account_id;
use sp_application_crypto::Pair;

/// Parachain Protocol Identifier
pub const INFRADID_PROTOCOL_ID: &str = "infradid";

/// Kusama Relaychain Local Network Identifier
pub const KUSAMA_RELAYCHAIN_LOCAL_NET: &str = "kusama-local";

/// Kusama Relaychain Development Network Identifier
pub const KUSAMA_RELAYCHAIN_DEV_NET: &str = "kusama-dev";

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = 2;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type InfraDIDChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Returns the [`Properties`] for the InfraDID parachain.
pub fn infradid_properties() -> Properties {
    let mut p = Properties::new();
    p.insert("ss58format".into(), constants::INFRADID_SS58PREFIX.into());
    p.insert("tokenDecimals".into(), constants::INFRADID_DECIMAL.into());
    p.insert(
        "tokenSymbol".into(),
        constants::INFRADID_TOKEN_SYMBOL.into(),
    );
    p
}

fn did_from_seed(did: &[u8; 32], seed: &[u8; 32]) -> (Did, DidKey) {
    let pk = sr25519::Pair::from_seed(seed).public().0;
    (
        Did(*did),
        DidKey::new_with_all_relationships(PublicKey::sr25519(pk)),
    )
}

/// Returns the InfraDID development chainspec.
pub fn infradid_development_config() -> InfraDIDChainSpec {
    InfraDIDChainSpec::from_genesis(
        "InfraDID Parachain Development",
        "infradid_dev",
        ChainType::Local,
        move || {
            infradid_dev_genesis(
                vec![(
                    unchecked_account_id::<sr25519::Public>("Alice"),
                    SessionKeys::from_seed_unchecked("Alice"),
                )],
                unchecked_account_id::<sr25519::Public>("Alice"),
                vec![
                    unchecked_account_id::<sr25519::Public>("Alice"),
                    unchecked_account_id::<sr25519::Public>("Bob"),
                    unchecked_account_id::<sr25519::Public>("Alice//stash"),
                    unchecked_account_id::<sr25519::Public>("Bob//stash"),
                ],
                Membership {
                    members: [
                        b"Alice\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                        b"Bob\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                        b"Charlie\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                    ]
                    .iter()
                    .map(|d| Did(**d))
                    .collect(),
                    vote_requirement: 2,
                },
                [
                    (
                        b"Alice\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                        b"Alicesk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                    ),
                    (
                        b"Bob\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                        b"Bobsk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                    ),
                    (
                        b"Charlie\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                        b"Charliesk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                    ),
                ]
                .iter()
                .map(|(name, sk)| did_from_seed(name, sk))
                .collect(),
            )
        },
        vec![],
        None,
        Some(INFRADID_PROTOCOL_ID),
        None,
        Some(infradid_properties()),
        Extensions {
            relay_chain: "".into(),
            para_id: INFRADID_PARACHAIN_ID,
        },
    )
}

/// Returns the InfraDID local chainspec.
pub fn infradid_local_config(localdev: bool) -> InfraDIDChainSpec {
    let id = if localdev {
        "infradid_localdev"
    } else {
        "infradid_local"
    };
    InfraDIDChainSpec::from_genesis(
        "InfraDID Parachain Local",
        id,
        ChainType::Local,
        move || {
            let invulnerables = if localdev {
                vec![(
                    unchecked_account_id::<sr25519::Public>("Alice"),
                    SessionKeys::from_seed_unchecked("Alice"),
                )]
            } else {
                vec![
                    (
                        unchecked_account_id::<sr25519::Public>("Alice"),
                        SessionKeys::from_seed_unchecked("Alice"),
                    ),
                    (
                        unchecked_account_id::<sr25519::Public>("Bob"),
                        SessionKeys::from_seed_unchecked("Bob"),
                    ),
                    (
                        unchecked_account_id::<sr25519::Public>("Charlie"),
                        SessionKeys::from_seed_unchecked("Charlie"),
                    ),
                    (
                        unchecked_account_id::<sr25519::Public>("Dave"),
                        SessionKeys::from_seed_unchecked("Dave"),
                    ),
                    (
                        unchecked_account_id::<sr25519::Public>("Eve"),
                        SessionKeys::from_seed_unchecked("Eve"),
                    ),
                ]
            };
            infradid_dev_genesis(
                invulnerables,
                unchecked_account_id::<sr25519::Public>("Alice"),
                vec![
                    unchecked_account_id::<sr25519::Public>("Alice"),
                    unchecked_account_id::<sr25519::Public>("Bob"),
                    unchecked_account_id::<sr25519::Public>("Charlie"),
                    unchecked_account_id::<sr25519::Public>("Dave"),
                    unchecked_account_id::<sr25519::Public>("Eve"),
                    unchecked_account_id::<sr25519::Public>("Alice//stash"),
                    unchecked_account_id::<sr25519::Public>("Bob//stash"),
                    unchecked_account_id::<sr25519::Public>("Charlie//stash"),
                    unchecked_account_id::<sr25519::Public>("Dave//stash"),
                    unchecked_account_id::<sr25519::Public>("Eve//stash"),
                ],
                Membership {
                    members: [
                        b"Alice\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                        b"Bob\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                        b"Charlie\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                    ]
                    .iter()
                    .map(|d| Did(**d))
                    .collect(),
                    vote_requirement: 2,
                },
                [
                    (
                        b"Alice\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                        b"Alicesk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                    ),
                    (
                        b"Bob\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                        b"Bobsk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                    ),
                    (
                        b"Charlie\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                        b"Charliesk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                    ),
                ]
                .iter()
                .map(|(name, sk)| did_from_seed(name, sk))
                .collect(),
            )
        },
        vec![],
        None,
        Some(INFRADID_PROTOCOL_ID),
        None,
        Some(infradid_properties()),
        Extensions {
            relay_chain: "".into(),
            para_id: INFRADID_PARACHAIN_ID,
        },
    )
}

fn infradid_dev_genesis(
    invulnerables: Vec<(AccountId, SessionKeys)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    master: Membership,
    dids: Vec<(Did, DidKey)>,
) -> GenesisConfig {
    GenesisConfig {
        system: infra_did_runtime::SystemConfig {
            code: infra_did_runtime::WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
        },
        balances: infra_did_runtime::BalancesConfig {
            balances: endowed_accounts[..endowed_accounts.len() / 2]
                .iter()
                .map(|k| {
                    (
                        k.clone(),
                        100 * INFRADID_ENDOWMENT / ((endowed_accounts.len() / 2) as Balance),
                    )
                })
                .collect(),
        },
        // no need to pass anything to aura, in fact it will panic if we do. Session will take care
        // of this.
        aura: Default::default(),
        sudo: infra_did_runtime::SudoConfig {
            key: Some(root_key),
        },
        parachain_info: infra_did_runtime::ParachainInfoConfig {
            parachain_id: INFRADID_PARACHAIN_ID.into(),
        },
        collator_selection: infra_did_runtime::CollatorSelectionConfig {
            invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
            candidacy_bond: IDID * 1000, // How many tokens will be reserved as collator
            ..Default::default()
        },
        session: infra_did_runtime::SessionConfig {
            keys: invulnerables
                .iter()
                .cloned()
                .map(|(acc, session_keys)| {
                    (
                        acc.clone(),  // account id
                        acc,          // validator id
                        session_keys, // collator session keys
                    )
                })
                .collect(),
        },
        democracy: DemocracyConfig::default(),
        council: CouncilConfig {
            members: endowed_accounts.iter().take(1).cloned().collect(),
            phantom: Default::default(),
        },
        technical_committee: TechnicalCommitteeConfig {
            members: endowed_accounts.iter().take(1).cloned().collect(),
            phantom: Default::default(),
        },
        asset_manager: Default::default(),
        council_membership: Default::default(),
        technical_membership: Default::default(),
        parachain_system: Default::default(),
        polkadot_xcm: infra_did_runtime::PolkadotXcmConfig {
            safe_xcm_version: Some(SAFE_XCM_VERSION),
        },
        master: MasterConfig { members: master },
        did_module: DIDModuleConfig { dids },
    }
}

// /// Returns the InfraDID testnet chainspec.
// pub fn infradid_testnet_config() -> Result<InfraDIDChainSpec, String> {
//     let mut spec = InfraDIDChainSpec::from_json_bytes(
//         &include_bytes!("../../../genesis/infradid-testnet-genesis.json")[..],
//     )?;
//     spec.extensions_mut().para_id = PARACHAIN_ID;
//     Ok(spec)
// }

// pub fn infradid_2085_config() -> Result<InfraDIDChainSpec, String> {
//     let mut spec = InfraDIDChainSpec::from_json_bytes(
//         &include_bytes!("../../../genesis/infradid-2085-genesis.json")[..],
//     )?;
//     spec.extensions_mut().para_id = INFRADID_ON_BAIKAL_PARACHAIN_ID;
//     Ok(spec)
// }

// /// Returns the InfraDID V3 2085 staging chainspec.
// pub fn infradid_v3_2085_staging_config() -> Result<InfraDIDChainSpec, String> {
//     let mut spec = InfraDIDChainSpec::from_json_bytes(
//         &include_bytes!("../../../genesis/infradid-v3-2085-genesis.json")[..],
//     )?;
//     spec.extensions_mut().para_id = 9997;
//     Ok(spec)
// }
