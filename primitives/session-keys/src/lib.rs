//! Session Key Primitives

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![forbid(rustdoc::broken_intra_doc_links)]
#![forbid(missing_docs)]

pub mod aura;
pub mod nimbus;
pub mod vrf;

pub use aura::AuraId;
pub use nimbus::NimbusId;
pub use vrf::VrfId;

#[cfg(feature = "std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
pub mod util;
