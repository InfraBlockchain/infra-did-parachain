# bbs_plus

## Overview

- Used to create and remove signature parameters and public keys.
- The public keys can either refer the signature params or not pass the reference while creating.
- The params and public keys are owned by a DID and can be only removed by that DID.

## Runtime

```rust
parameter_types! {
    // 128 bytes, for large labels, hash of a label can be used
    pub const LabelMaxSize: u32 = 128;
    pub const LabelPerByteWeight: Weight = Weight::from_ref_time(10);
    // 16KB
    pub const ParamsMaxSize: u32 = 65536;
    pub const ParamsPerByteWeight: Weight = Weight::from_ref_time(10);
    pub const PublicKeyMaxSize: u32 = 256;
    pub const PublicKeyPerByteWeight: Weight = Weight::from_ref_time(10);
}

impl bbs_plus::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type LabelMaxSize = LabelMaxSize;
    type LabelPerByteWeight = LabelPerByteWeight;
    type ParamsMaxSize = ParamsMaxSize;
    type ParamsPerByteWeight = ParamsPerByteWeight;
    type PublicKeyMaxSize = PublicKeyMaxSize;
    type PublicKeyPerByteWeight = PublicKeyPerByteWeight;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        /* snip */
        BbsPlus: bbs_plus::{Pallet, Call, Storage, Event},
        /* snip */
    }
);
```

## Call

### `addParams`

Add new BBS+ params.

---

### `addPublicKey`

Add a BBS+ public key. Only the DID controller can add key and it should use the nonce from the DID module.

This kind of key cannot be removed by calling `remove_keys` from the DID module but only by calling `remove_public_key` of this module.

---

### `removeParams`

Remove existing BBS+ params.

---

### `removePublicKey`

Remove BBS+ public key. Only the DID controller can remove key and it should use the nonce from the DID module.

This kind of key cannot be removed by calling `remove_keys` from the DID module.

---

## Query

### `bbsPlusKeys`

Public keys are stored as key value (did, counter) -> public key

Its assumed that the public keys are always members of G2. It does impact any logic on the chain but makes up for one less storage value

---

### `bbsPlusParams`

Parameters are stored as key value (did, counter) -> params

---

### `paramsCounter`

Pair of counters where each is used to assign unique id to parameters and public keys respectively. On adding new params or keys, corresponding counter is increased by 1 but the counters don't decrease on removal

---
