# attest

## Runtime

```rust
parameter_types! {
    // 8KB
    pub const StorageWeight: Weight = Weight::from_ref_time(1100);
}

impl attest::Config for Runtime {
    type StorageWeight = StorageWeight;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        /* snip */
        Attest: attest::{Pallet, Call, Storage},
        /* snip */
    }
);
```

## Call

### `setClaim`

Creates an attestation claim on chain for a specific DID

---

## Query

### `attestations`

The priority value provides replay protection and also gives attestations a partial ordering.

Signatures with lesser or equal priority to those previously posted by the same entity are not accepted by the chain.

---
