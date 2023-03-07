//! Parachain CLI

#![warn(missing_docs)]

extern crate alloc;

mod aura_or_nimbus_consensus;
pub mod builder;
pub mod chain_specs;
pub mod cli;
pub mod client;
pub mod command;
mod instant_finalize;
pub mod rpc;
pub mod service;
