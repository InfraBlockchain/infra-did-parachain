// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use parachains_common::{AccountId, AuraId, Signature, Balance as DIDBalance};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
    sc_service::GenericChainSpec<infra_did_parachain_runtime::RuntimeGenesisConfig, Extensions>;

const INFRA_RELAY_ED: DIDBalance = parachains_common::infra_relay::currency::EXISTENTIAL_DEPOSIT;

const PARACHAIN_ID: u32 = 1002;

const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
    get_from_seed::<AuraId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn session_keys(keys: AuraId) -> infra_did_parachain_runtime::SessionKeys {
    infra_did_parachain_runtime::SessionKeys { aura: keys }
}

pub fn development_config() -> ChainSpec {
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("ss58Format".into(), 0.into());
    properties.insert("tokenSymbol".into(), "".into());
    properties.insert("tokenDecimals".into(), 10.into());

    ChainSpec::from_genesis(
        // Name
        "InfraBlockchain DID Chain Dev",
        // ID
        "did-hub-infra-dev",
        ChainType::Development,
        move || {
            parachain_genesis(
                // initial collators.
                vec![(
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_collator_keys_from_seed("Alice"),
                )],
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                ],
                Some(get_account_id_from_seed::<sr25519::Public>("Alice")),
                PARACHAIN_ID.into(),
            )
        },
        Vec::new(),
        None,
        None,
        None,
        Some(properties),
        Extensions {
            relay_chain: "infra-relay-dev".into(),
            para_id: PARACHAIN_ID,
        },
    )
}

pub fn local_config() -> ChainSpec {
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("ss58Format".into(), 0.into());
    properties.insert("tokenSymbol".into(), "".into());
    properties.insert("tokenDecimals".into(), 10.into());

    ChainSpec::from_genesis(
        // Name
        "InfraBlockchain DID Chain Local",
        // ID
        "did-hub-infra-local",
        ChainType::Local,
        move || {
            parachain_genesis(
                // initial collators.
                vec![
                    (
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        get_collator_keys_from_seed("Alice"),
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                        get_collator_keys_from_seed("Bob"),
                    ),
                ],
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                ],
                Some(get_account_id_from_seed::<sr25519::Public>("Alice")),
                PARACHAIN_ID.into(),
            )
        },
        Vec::new(),
        None,
        None,
        None,
        Some(properties),
        Extensions {
            relay_chain: "infra-relay-local".into(),
            para_id: PARACHAIN_ID,
        },
    )
}

// Not used for syncing, but just to determine the genesis values set for the upgrade from shell.
pub fn mainnet_config() -> ChainSpec {
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("ss58Format".into(), 0.into());
    properties.insert("tokenSymbol".into(), "".into());
    properties.insert("tokenDecimals".into(), 10.into());

    ChainSpec::from_genesis(
        // Name
        "InfraBlockchain DID Chain Main",
        // ID
        "did-hub-infra",
        ChainType::Live,
        move || {
            parachain_genesis(
                // initial collators.
                vec![
                    (
                        hex!("4c3d674d2a01060f0ded218e5dcc6f90c1726f43df79885eb3e22d97a20d5421")
                            .into(),
                        hex!("4c3d674d2a01060f0ded218e5dcc6f90c1726f43df79885eb3e22d97a20d5421")
                            .unchecked_into(),
                    ),
                    (
                        hex!("c7d7d38d16bc23c6321152c50306212dc22c0efc04a2e52b5cccfc31ab3d7811")
                            .into(),
                        hex!("c7d7d38d16bc23c6321152c50306212dc22c0efc04a2e52b5cccfc31ab3d7811")
                            .unchecked_into(),
                    ),
                    (
                        hex!("c5c07ba203d7375675f5c1ebe70f0a5bb729ae57b48bcc877fcc2ab21309b762")
                            .into(),
                        hex!("c5c07ba203d7375675f5c1ebe70f0a5bb729ae57b48bcc877fcc2ab21309b762")
                            .unchecked_into(),
                    ),
                    (
                        hex!("0b2d0013fb974794bd7aa452465b567d48ef70373fe231a637c1fb7c547e85b3")
                            .into(),
                        hex!("0b2d0013fb974794bd7aa452465b567d48ef70373fe231a637c1fb7c547e85b3")
                            .unchecked_into(),
                    ),
                ],
                Default::default(),
                None,
                PARACHAIN_ID.into(),
            )
        },
        vec![],
        None,
        None,
        None,
        Some(properties),
        Extensions {
            relay_chain: "infra-relay".into(),
            para_id: PARACHAIN_ID,
        },
    )
}

fn parachain_genesis(
    invulnerables: Vec<(AccountId, AuraId)>,
    endowed_accounts: Vec<AccountId>,
    root_key: Option<AccountId>,
    id: ParaId,
) -> infra_did_parachain_runtime::RuntimeGenesisConfig {
    infra_did_parachain_runtime::RuntimeGenesisConfig {
        system: infra_did_parachain_runtime::SystemConfig {
            code: infra_did_parachain_runtime::WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
            ..Default::default()
        },
        balances: infra_did_parachain_runtime::BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, INFRA_RELAY_ED * 4096))
                .collect(),
        },
        parachain_info: infra_did_parachain_runtime::ParachainInfoConfig {
            parachain_id: id,
            ..Default::default()
        },
        collator_selection: infra_did_parachain_runtime::CollatorSelectionConfig {
            invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
            candidacy_bond: INFRA_RELAY_ED * 16,
            ..Default::default()
        },
        session: infra_did_parachain_runtime::SessionConfig {
            keys: invulnerables
                .into_iter()
                .map(|(acc, aura)| {
                    (
                        acc.clone(),        // account id
                        acc,                // validator id
                        session_keys(aura), // session keys
                    )
                })
                .collect(),
        },
        // no need to pass anything to aura, in fact it will panic if we do. Session will take care
        // of this.
        aura: Default::default(),
        aura_ext: Default::default(),
        parachain_system: Default::default(),
        infra_xcm: infra_did_parachain_runtime::InfraXcmConfig {
            safe_xcm_version: Some(SAFE_XCM_VERSION),
            ..Default::default()
        },
        sudo: infra_did_parachain_runtime::SudoConfig { key: root_key },
        did_module: Default::default(),
        ..Default::default()
    }
}
