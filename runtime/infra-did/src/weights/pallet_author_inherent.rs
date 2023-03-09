

//! Autogenerated weights for pallet_author_inherent
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("infradid-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/infradid
// benchmark
// pallet
// --chain=infradid-dev
// --steps=50
// --repeat=20
// --pallet=pallet_author_inherent
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_author_inherent.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_author_inherent.
pub trait WeightInfo {
    fn kick_off_authorship_validation() -> Weight;
}

/// Weights for pallet_author_inherent using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_author_inherent::WeightInfo for SubstrateWeight<T> {
    // Storage: ParachainSystem ValidationData (r:1 w:0)
    // Storage: AuthorInherent HighestSlotSeen (r:1 w:1)
    // Storage: AuthorInherent Author (r:1 w:0)
    // Storage: Session NextKeys (r:1 w:0)
    // Storage: Session Validators (r:1 w:0)
    fn kick_off_authorship_validation() -> Weight {
        Weight::from_ref_time(16_985_000)
            .saturating_add(T::DbWeight::get().reads(5 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    // Storage: ParachainSystem ValidationData (r:1 w:0)
    // Storage: AuthorInherent HighestSlotSeen (r:1 w:1)
    // Storage: AuthorInherent Author (r:1 w:0)
    // Storage: Session NextKeys (r:1 w:0)
    // Storage: Session Validators (r:1 w:0)
    fn kick_off_authorship_validation() -> Weight {
        Weight::from_ref_time(16_985_000)
            .saturating_add(RocksDbWeight::get().reads(5 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
}