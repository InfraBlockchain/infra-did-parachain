//! Chain Specification Definitions

// NOTE: Tolerate clippy warning originating in ChainSpecGroup, which is a dependency.
#![allow(clippy::derive_partial_eq_without_eq)]
// NOTE: Missing documentation on all `ChainSpecGroup` implementations.
#![allow(missing_docs)]

use common_primitives::{
    constants,
    types::{AccountId, Balance},
};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::{ChainType, Properties};
use serde::{Deserialize, Serialize};
use sp_core::sr25519;

pub mod infra_did;

pub use self::infra_did::*;
pub use infra_did_runtime::currency::IDID;

/// InfraDID Endowment: 10 endowment so that total supply is 10B
pub const INFRADID_ENDOWMENT: Balance = 1_000_000_000 * IDID;

/// Infra DID Network Chain Spec
pub type ChainSpec = sc_service::GenericChainSpec<infra_did::GenesisConfig, Extensions>;

/// The extensions for the [`ChainSpec`].
#[derive(
    ChainSpecExtension, ChainSpecGroup, Clone, Debug, Deserialize, Eq, PartialEq, Serialize,
)]
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
