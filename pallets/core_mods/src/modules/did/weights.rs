//! Autogenerated weights for did
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2022-08-01, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Native), WASM-EXECUTION: Interpreted, CHAIN: Some("mainnet"), DB CACHE: 128

// Executed Command:
// ./target/production/dock-node
// benchmark
// --execution=native
// --chain=mainnet
// --pallet=did
// --extra
// --extrinsic=*
// --repeat=20
// --steps=50
// --template=node/module-weight-template.hbs
// --output=./pallets/core_mods/src/modules/did/weights.rs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for did.
pub trait WeightInfo {
    fn add_keys_sr25519(k: u32) -> Weight;
    fn add_keys_ed25519(k: u32) -> Weight;
    fn remove_keys_sr25519(k: u32) -> Weight;
    fn remove_keys_ed25519(k: u32) -> Weight;
    fn add_controllers_sr25519(k: u32) -> Weight;
    fn add_controllers_ed25519(k: u32) -> Weight;
    fn remove_controllers_sr25519(k: u32) -> Weight;
    fn remove_controllers_ed25519(k: u32) -> Weight;
    fn add_service_endpoint_sr25519(o: u32, l: u32, i: u32) -> Weight;
    fn add_service_endpoint_ed25519(o: u32, l: u32, i: u32) -> Weight;
    fn remove_service_endpoint_sr25519(i: u32) -> Weight;
    fn remove_service_endpoint_ed25519(i: u32) -> Weight;
    fn remove_onchain_did_sr25519() -> Weight;
    fn remove_onchain_did_ed25519() -> Weight;
    fn new_onchain(k: u32, c: u32) -> Weight;
    fn new_offchain(k: u32) -> Weight;
    fn set_offchain_did_doc_ref(k: u32) -> Weight;
    fn remove_offchain_did() -> Weight;
}

/// Weights for did using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn add_keys_sr25519(k: u32) -> Weight {
        Weight::from_ref_time(56_930_000 as u64)
            // Standard Error: 34_000
            .saturating_add(Weight::from_ref_time(1_072_000 as u64).saturating_mul(k as u64))
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
    }
    fn add_keys_ed25519(k: u32) -> Weight {
        Weight::from_ref_time(57_701_000 as u64)
            // Standard Error: 29_000
            .saturating_add(Weight::from_ref_time(712_000 as u64).saturating_mul(k as u64))
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
    }
    fn remove_keys_sr25519(k: u32) -> Weight {
        Weight::from_ref_time(62_781_000 as u64)
            // Standard Error: 26_000
            .saturating_add(Weight::from_ref_time(2_804_000 as u64).saturating_mul(k as u64))
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(k as u64)))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
    }
    fn remove_keys_ed25519(k: u32) -> Weight {
        Weight::from_ref_time(60_847_000 as u64)
            // Standard Error: 30_000
            .saturating_add(Weight::from_ref_time(2_717_000 as u64).saturating_mul(k as u64))
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(k as u64)))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
    }
    fn add_controllers_sr25519(k: u32) -> Weight {
        Weight::from_ref_time(60_892_000 as u64)
            // Standard Error: 34_000
            .saturating_add(Weight::from_ref_time(3_351_000 as u64).saturating_mul(k as u64))
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(k as u64)))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
    }
    fn add_controllers_ed25519(k: u32) -> Weight {
        Weight::from_ref_time(60_140_000 as u64)
            // Standard Error: 59_000
            .saturating_add(Weight::from_ref_time(3_061_000 as u64).saturating_mul(k as u64))
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(k as u64)))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
    }
    fn remove_controllers_sr25519(k: u32) -> Weight {
        Weight::from_ref_time(60_478_000 as u64)
            // Standard Error: 119_000
            .saturating_add(Weight::from_ref_time(3_625_000 as u64).saturating_mul(k as u64))
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(k as u64)))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
    }
    fn remove_controllers_ed25519(k: u32) -> Weight {
        Weight::from_ref_time(60_970_000 as u64)
            // Standard Error: 43_000
            .saturating_add(Weight::from_ref_time(3_273_000 as u64).saturating_mul(k as u64))
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(k as u64)))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
    }
    fn add_service_endpoint_sr25519(o: u32, l: u32, i: u32) -> Weight {
        Weight::from_ref_time(62_630_000 as u64)
            // Standard Error: 15_000
            .saturating_add(Weight::from_ref_time(146_000 as u64).saturating_mul(o as u64))
            // Standard Error: 15_000
            .saturating_add(Weight::from_ref_time(82_000 as u64).saturating_mul(l as u64))
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(3_000 as u64).saturating_mul(i as u64))
            .saturating_add(T::DbWeight::get().reads(5 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    fn add_service_endpoint_ed25519(o: u32, l: u32, i: u32) -> Weight {
        Weight::from_ref_time(61_631_000 as u64)
            // Standard Error: 16_000
            .saturating_add(Weight::from_ref_time(60_000 as u64).saturating_mul(o as u64))
            // Standard Error: 16_000
            .saturating_add(Weight::from_ref_time(58_000 as u64).saturating_mul(l as u64))
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(4_000 as u64).saturating_mul(i as u64))
            .saturating_add(T::DbWeight::get().reads(5 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    fn remove_service_endpoint_sr25519(i: u32) -> Weight {
        Weight::from_ref_time(62_886_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(28_000 as u64).saturating_mul(i as u64))
            .saturating_add(T::DbWeight::get().reads(5 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    fn remove_service_endpoint_ed25519(i: u32) -> Weight {
        Weight::from_ref_time(62_262_000 as u64)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(9_000 as u64).saturating_mul(i as u64))
            .saturating_add(T::DbWeight::get().reads(5 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    fn remove_onchain_did_sr25519() -> Weight {
        Weight::from_ref_time(85_904_000 as u64)
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().writes(34 as u64))
    }
    fn remove_onchain_did_ed25519() -> Weight {
        Weight::from_ref_time(86_475_000 as u64)
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().writes(34 as u64))
    }
    fn new_onchain(k: u32, c: u32) -> Weight {
        Weight::from_ref_time(10_672_000 as u64)
            // Standard Error: 4_000
            .saturating_add(Weight::from_ref_time(549_000 as u64).saturating_mul(k as u64))
            // Standard Error: 4_000
            .saturating_add(Weight::from_ref_time(606_000 as u64).saturating_mul(c as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
    }
    fn new_offchain(k: u32) -> Weight {
        Weight::from_ref_time(9_652_000 as u64)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(k as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    fn set_offchain_did_doc_ref(k: u32) -> Weight {
        Weight::from_ref_time(10_356_000 as u64)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(1_000 as u64).saturating_mul(k as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    fn remove_offchain_did() -> Weight {
        Weight::from_ref_time(10_057_000 as u64)
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn add_keys_sr25519(k: u32) -> Weight {
        Weight::from_ref_time(56_930_000 as u64)
            // Standard Error: 34_000
            .saturating_add(Weight::from_ref_time(1_072_000 as u64).saturating_mul(k as u64))
            .saturating_add(RocksDbWeight::get().reads(4 as u64))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
            .saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
    }
    fn add_keys_ed25519(k: u32) -> Weight {
        Weight::from_ref_time(57_701_000 as u64)
            // Standard Error: 29_000
            .saturating_add(Weight::from_ref_time(712_000 as u64).saturating_mul(k as u64))
            .saturating_add(RocksDbWeight::get().reads(4 as u64))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
            .saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
    }
    fn remove_keys_sr25519(k: u32) -> Weight {
        Weight::from_ref_time(62_781_000 as u64)
            // Standard Error: 26_000
            .saturating_add(Weight::from_ref_time(2_804_000 as u64).saturating_mul(k as u64))
            .saturating_add(RocksDbWeight::get().reads(4 as u64))
            .saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(k as u64)))
            .saturating_add(RocksDbWeight::get().writes(4 as u64))
            .saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
    }
    fn remove_keys_ed25519(k: u32) -> Weight {
        Weight::from_ref_time(60_847_000 as u64)
            // Standard Error: 30_000
            .saturating_add(Weight::from_ref_time(2_717_000 as u64).saturating_mul(k as u64))
            .saturating_add(RocksDbWeight::get().reads(4 as u64))
            .saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(k as u64)))
            .saturating_add(RocksDbWeight::get().writes(4 as u64))
            .saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
    }
    fn add_controllers_sr25519(k: u32) -> Weight {
        Weight::from_ref_time(60_892_000 as u64)
            // Standard Error: 34_000
            .saturating_add(Weight::from_ref_time(3_351_000 as u64).saturating_mul(k as u64))
            .saturating_add(RocksDbWeight::get().reads(4 as u64))
            .saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(k as u64)))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
            .saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
    }
    fn add_controllers_ed25519(k: u32) -> Weight {
        Weight::from_ref_time(60_140_000 as u64)
            // Standard Error: 59_000
            .saturating_add(Weight::from_ref_time(3_061_000 as u64).saturating_mul(k as u64))
            .saturating_add(RocksDbWeight::get().reads(4 as u64))
            .saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(k as u64)))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
            .saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
    }
    fn remove_controllers_sr25519(k: u32) -> Weight {
        Weight::from_ref_time(60_478_000 as u64)
            // Standard Error: 119_000
            .saturating_add(Weight::from_ref_time(3_625_000 as u64).saturating_mul(k as u64))
            .saturating_add(RocksDbWeight::get().reads(4 as u64))
            .saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(k as u64)))
            .saturating_add(RocksDbWeight::get().writes(3 as u64))
            .saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
    }
    fn remove_controllers_ed25519(k: u32) -> Weight {
        Weight::from_ref_time(60_970_000 as u64)
            // Standard Error: 43_000
            .saturating_add(Weight::from_ref_time(3_273_000 as u64).saturating_mul(k as u64))
            .saturating_add(RocksDbWeight::get().reads(4 as u64))
            .saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(k as u64)))
            .saturating_add(RocksDbWeight::get().writes(3 as u64))
            .saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
    }
    fn add_service_endpoint_sr25519(o: u32, l: u32, i: u32) -> Weight {
        Weight::from_ref_time(62_630_000 as u64)
            // Standard Error: 15_000
            .saturating_add(Weight::from_ref_time(146_000 as u64).saturating_mul(o as u64))
            // Standard Error: 15_000
            .saturating_add(Weight::from_ref_time(82_000 as u64).saturating_mul(l as u64))
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(3_000 as u64).saturating_mul(i as u64))
            .saturating_add(RocksDbWeight::get().reads(5 as u64))
            .saturating_add(RocksDbWeight::get().writes(3 as u64))
    }
    fn add_service_endpoint_ed25519(o: u32, l: u32, i: u32) -> Weight {
        Weight::from_ref_time(61_631_000 as u64)
            // Standard Error: 16_000
            .saturating_add(Weight::from_ref_time(60_000 as u64).saturating_mul(o as u64))
            // Standard Error: 16_000
            .saturating_add(Weight::from_ref_time(58_000 as u64).saturating_mul(l as u64))
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(4_000 as u64).saturating_mul(i as u64))
            .saturating_add(RocksDbWeight::get().reads(5 as u64))
            .saturating_add(RocksDbWeight::get().writes(3 as u64))
    }
    fn remove_service_endpoint_sr25519(i: u32) -> Weight {
        Weight::from_ref_time(62_886_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(28_000 as u64).saturating_mul(i as u64))
            .saturating_add(RocksDbWeight::get().reads(5 as u64))
            .saturating_add(RocksDbWeight::get().writes(3 as u64))
    }
    fn remove_service_endpoint_ed25519(i: u32) -> Weight {
        Weight::from_ref_time(62_262_000 as u64)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(9_000 as u64).saturating_mul(i as u64))
            .saturating_add(RocksDbWeight::get().reads(5 as u64))
            .saturating_add(RocksDbWeight::get().writes(3 as u64))
    }
    fn remove_onchain_did_sr25519() -> Weight {
        Weight::from_ref_time(85_904_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(4 as u64))
            .saturating_add(RocksDbWeight::get().writes(34 as u64))
    }
    fn remove_onchain_did_ed25519() -> Weight {
        Weight::from_ref_time(86_475_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(4 as u64))
            .saturating_add(RocksDbWeight::get().writes(34 as u64))
    }
    fn new_onchain(k: u32, c: u32) -> Weight {
        Weight::from_ref_time(10_672_000 as u64)
            // Standard Error: 4_000
            .saturating_add(Weight::from_ref_time(549_000 as u64).saturating_mul(k as u64))
            // Standard Error: 4_000
            .saturating_add(Weight::from_ref_time(606_000 as u64).saturating_mul(c as u64))
            .saturating_add(RocksDbWeight::get().reads(2 as u64))
            .saturating_add(RocksDbWeight::get().writes(3 as u64))
            .saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
            .saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
    }
    fn new_offchain(k: u32) -> Weight {
        Weight::from_ref_time(9_652_000 as u64)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(k as u64))
            .saturating_add(RocksDbWeight::get().reads(2 as u64))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
    }
    fn set_offchain_did_doc_ref(k: u32) -> Weight {
        Weight::from_ref_time(10_356_000 as u64)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(1_000 as u64).saturating_mul(k as u64))
            .saturating_add(RocksDbWeight::get().reads(2 as u64))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
    }
    fn remove_offchain_did() -> Weight {
        Weight::from_ref_time(10_057_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(2 as u64))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
    }
}
