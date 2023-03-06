// Copyright 2020-2023 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for collator_selection
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("calamari-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/manta
// benchmark
// pallet
// --chain=calamari-dev
// --steps=50
// --repeat=20
// --pallet=collator_selection
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/collator_selection.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for collator_selection.
pub trait WeightInfo {
    fn set_invulnerables(b: u32, ) -> Weight;
    fn set_desired_candidates() -> Weight;
    fn set_candidacy_bond() -> Weight;
    fn set_eviction_baseline() -> Weight;
    fn set_eviction_tolerance() -> Weight;
    fn register_as_candidate(c: u32, ) -> Weight;
    fn leave_intent(c: u32, ) -> Weight;
    fn remove_collator(c: u32, ) -> Weight;
    fn register_candidate(c: u32, ) -> Weight;
    fn note_author() -> Weight;
    fn new_session(c: u32, ) -> Weight;
}

/// Weights for collator_selection using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    // Storage: CollatorSelection Invulnerables (r:0 w:1)
    fn set_invulnerables(b: u32, ) -> Weight {
        Weight::from_ref_time(9_758_000)
            // Standard Error: 12_000
            .saturating_add(Weight::from_ref_time(99_000).saturating_mul(b as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection DesiredCandidates (r:0 w:1)
    fn set_desired_candidates() -> Weight {
        Weight::from_ref_time(11_283_000)
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection CandidacyBond (r:0 w:1)
    fn set_candidacy_bond() -> Weight {
        Weight::from_ref_time(9_598_000)
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection EvictionBaseline (r:0 w:1)
    fn set_eviction_baseline() -> Weight {
        Weight::from_ref_time(9_374_000)
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection EvictionTolerance (r:0 w:1)
    fn set_eviction_tolerance() -> Weight {
        Weight::from_ref_time(9_330_000)
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection Candidates (r:1 w:1)
    // Storage: CollatorSelection DesiredCandidates (r:1 w:0)
    // Storage: CollatorSelection Invulnerables (r:1 w:0)
    // Storage: Session NextKeys (r:1 w:0)
    // Storage: CollatorSelection CandidacyBond (r:1 w:0)
    fn register_as_candidate(c: u32, ) -> Weight {
        Weight::from_ref_time(44_181_000)
            // Standard Error: 4_000
            .saturating_add(Weight::from_ref_time(477_000).saturating_mul(c as u64))
            .saturating_add(T::DbWeight::get().reads(5 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection Candidates (r:1 w:1)
    fn leave_intent(c: u32, ) -> Weight {
        Weight::from_ref_time(30_937_000)
            // Standard Error: 3_000
            .saturating_add(Weight::from_ref_time(457_000).saturating_mul(c as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection Invulnerables (r:1 w:0)
    // Storage: CollatorSelection Candidates (r:1 w:1)
    fn remove_collator(c: u32, ) -> Weight {
        Weight::from_ref_time(30_209_000)
            // Standard Error: 7_000
            .saturating_add(Weight::from_ref_time(295_000).saturating_mul(c as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection Candidates (r:1 w:1)
    // Storage: CollatorSelection DesiredCandidates (r:1 w:0)
    // Storage: CollatorSelection Invulnerables (r:1 w:0)
    // Storage: Session NextKeys (r:1 w:0)
    // Storage: CollatorSelection CandidacyBond (r:1 w:0)
    fn register_candidate(c: u32, ) -> Weight {
        Weight::from_ref_time(40_356_000)
            // Standard Error: 8_000
            .saturating_add(Weight::from_ref_time(336_000).saturating_mul(c as u64))
            .saturating_add(T::DbWeight::get().reads(5 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: System Account (r:2 w:2)
    // Storage: CollatorSelection BlocksPerCollatorThisSession (r:1 w:1)
    // Storage: System BlockWeight (r:1 w:1)
    fn note_author() -> Weight {
        Weight::from_ref_time(34_463_000)
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
    }
    // Storage: CollatorSelection Candidates (r:1 w:0)
    // Storage: CollatorSelection EvictionBaseline (r:1 w:0)
    // Storage: CollatorSelection EvictionTolerance (r:1 w:0)
    // Storage: CollatorSelection BlocksPerCollatorThisSession (r:2 w:2)
    // Storage: CollatorSelection Invulnerables (r:1 w:0)
    // Storage: System BlockWeight (r:1 w:1)
    // Storage: Session Validators (r:1 w:0)
    // Storage: System Account (r:1 w:1)
    fn new_session(c: u32, ) -> Weight {
        Weight::from_ref_time(11_539_000)
            // Standard Error: 84_000
            .saturating_add(Weight::from_ref_time(24_290_000).saturating_mul(c as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(c as u64)))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    // Storage: CollatorSelection Invulnerables (r:0 w:1)
    fn set_invulnerables(b: u32, ) -> Weight {
        Weight::from_ref_time(9_758_000)
            // Standard Error: 12_000
            .saturating_add(Weight::from_ref_time(99_000).saturating_mul(b as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection DesiredCandidates (r:0 w:1)
    fn set_desired_candidates() -> Weight {
        Weight::from_ref_time(11_283_000)
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection CandidacyBond (r:0 w:1)
    fn set_candidacy_bond() -> Weight {
        Weight::from_ref_time(9_598_000)
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection EvictionBaseline (r:0 w:1)
    fn set_eviction_baseline() -> Weight {
        Weight::from_ref_time(9_374_000)
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection EvictionTolerance (r:0 w:1)
    fn set_eviction_tolerance() -> Weight {
        Weight::from_ref_time(9_330_000)
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection Candidates (r:1 w:1)
    // Storage: CollatorSelection DesiredCandidates (r:1 w:0)
    // Storage: CollatorSelection Invulnerables (r:1 w:0)
    // Storage: Session NextKeys (r:1 w:0)
    // Storage: CollatorSelection CandidacyBond (r:1 w:0)
    fn register_as_candidate(c: u32, ) -> Weight {
        Weight::from_ref_time(44_181_000)
            // Standard Error: 4_000
            .saturating_add(Weight::from_ref_time(477_000).saturating_mul(c as u64))
            .saturating_add(RocksDbWeight::get().reads(5 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection Candidates (r:1 w:1)
    fn leave_intent(c: u32, ) -> Weight {
        Weight::from_ref_time(30_937_000)
            // Standard Error: 3_000
            .saturating_add(Weight::from_ref_time(457_000).saturating_mul(c as u64))
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection Invulnerables (r:1 w:0)
    // Storage: CollatorSelection Candidates (r:1 w:1)
    fn remove_collator(c: u32, ) -> Weight {
        Weight::from_ref_time(30_209_000)
            // Standard Error: 7_000
            .saturating_add(Weight::from_ref_time(295_000).saturating_mul(c as u64))
            .saturating_add(RocksDbWeight::get().reads(2 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: CollatorSelection Candidates (r:1 w:1)
    // Storage: CollatorSelection DesiredCandidates (r:1 w:0)
    // Storage: CollatorSelection Invulnerables (r:1 w:0)
    // Storage: Session NextKeys (r:1 w:0)
    // Storage: CollatorSelection CandidacyBond (r:1 w:0)
    fn register_candidate(c: u32, ) -> Weight {
        Weight::from_ref_time(40_356_000)
            // Standard Error: 8_000
            .saturating_add(Weight::from_ref_time(336_000).saturating_mul(c as u64))
            .saturating_add(RocksDbWeight::get().reads(5 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: System Account (r:2 w:2)
    // Storage: CollatorSelection BlocksPerCollatorThisSession (r:1 w:1)
    // Storage: System BlockWeight (r:1 w:1)
    fn note_author() -> Weight {
        Weight::from_ref_time(34_463_000)
            .saturating_add(RocksDbWeight::get().reads(4 as u64))
            .saturating_add(RocksDbWeight::get().writes(4 as u64))
    }
    // Storage: CollatorSelection Candidates (r:1 w:0)
    // Storage: CollatorSelection EvictionBaseline (r:1 w:0)
    // Storage: CollatorSelection EvictionTolerance (r:1 w:0)
    // Storage: CollatorSelection BlocksPerCollatorThisSession (r:2 w:2)
    // Storage: CollatorSelection Invulnerables (r:1 w:0)
    // Storage: System BlockWeight (r:1 w:1)
    // Storage: Session Validators (r:1 w:0)
    // Storage: System Account (r:1 w:1)
    fn new_session(c: u32, ) -> Weight {
        Weight::from_ref_time(11_539_000)
            // Standard Error: 84_000
            .saturating_add(Weight::from_ref_time(24_290_000).saturating_mul(c as u64))
            .saturating_add(RocksDbWeight::get().reads(6 as u64))
            .saturating_add(RocksDbWeight::get().reads((2 as u64).saturating_mul(c as u64)))
            .saturating_add(RocksDbWeight::get().writes(3 as u64))
            .saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
    }
}
