# accumulator

## Overview

accumulator public keys and some accumulator details like current accumulated value, last updated, etc. They are somewhat agnostic
to the cryptographic details and treat the values as bytes with some size bounds.

- The parameters and public keys are managed in the same way as BBS+ signatures.
- Accumulators are owned by a DID and can be only removed by that DID.
- Accumulators are identified by a unique id and that id is used to send updates or remove it.
- The accumulator update contains the additions, removals and the witness update info and these are not stored in chain
  state but are present in the blocks and the accumulated value corresponding to the update is logged in the event.
- In the chain state, only the most recent accumulated value is stored (along with some metadata like creation time,
  last update, etc), which is sufficient to verify the witness or the proof of knowledge.
- To update the witness, the updates and witness update info should be parsed from the blocks and the accumulator module provides the functions get the updates and necessary events from the block

## Runtime

```rust
parameter_types! {
    // 128 bytes, for large labels, hash of a label can be used
    pub const LabelMaxSize: u32 = 128;
    pub const LabelPerByteWeight: Weight = Weight::from_ref_time(10);
    // 16KB
    pub const PublicKeyMaxSize: u32 = 256;
    pub const PublicKeyPerByteWeight: Weight = Weight::from_ref_time(10);
    pub const AccumulatorParamsMaxSize: u32 = 512;
    pub const AccumulatorParamsPerByteWeight: Weight = Weight::from_ref_time(10);
    pub const AccumulatedMaxSize: u32 = 128;
    pub const AccumulatedPerByteWeight: Weight = Weight::from_ref_time(10);
}

impl accumulator::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type LabelMaxSize = LabelMaxSize;
    type LabelPerByteWeight = LabelPerByteWeight;
    type ParamsMaxSize = AccumulatorParamsMaxSize;
    type ParamsPerByteWeight = AccumulatorParamsPerByteWeight;
    type PublicKeyMaxSize = PublicKeyMaxSize;
    type PublicKeyPerByteWeight = PublicKeyPerByteWeight;
    type AccumulatedMaxSize = AccumulatedMaxSize;
    type AccumulatedPerByteWeight = AccumulatedPerByteWeight;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
      /* snip */
      Accumulator: accumulator::{Pallet, Call, Storage, Event},
      /* snip */
    }
);
```

## Call

### `addAccumulator`

Add a new accumulator with the initial accumulated value. Each accumulator has a unique id and it refers to a public key. It is assumed that the accumulator is owned by the DID that owns the public key.

It logs an event with the accumulator id and accumulated value. For each new accumulator, its creation block is recorded in state to indicate from which block, the chain should be scanned for the accumulator's updates.

---

### `addParams`

Add accumulator parameters

---

### `addPublicKey`

Add accumulator public key

---

### `removeAccumulator`

Remove the accumulator from chain. This frees up the id for reuse.

---

### `removeParams`

remove accumulator parameters

---

### `removePublicKey`

Remove accumulator public key

---

### `updateAccumulator`

Update an existing accumulator. The update contains the new accumulated value, the updates themselves and the witness updated info. The updates and witness update info are optional as the owner might be privately communicating the updated witnesses. It logs an event with the accumulator id and the new accumulated value which is sufficient for a verifier. But the prover (who has a witness to update) needs the updates and the witness update info and is expected to look into the corresponding extrinsic arguments.

---

## Query

### `AccumulatorOwnerCounters`

---

### `AccumulatorParams`

---

### `AccumulatorKeys`

Public key storage is kept separate from accumulator storage and a single key can be used to manage several accumulators. It is assumed that whoever (DID) owns the public key, owns the accumulator as well and only that DID can update accumulator.

---

### `Accumulators`

Stores latest accumulator as key value: accumulator id -> (created_at, last_updated_at, Accumulator) `created_at` is the block number when the accumulator was created and is intended to serve as a starting point for anyone looking for all updates to the accumulator. `last_updated_at` is the block number when the last update was sent. `created_at` and `last_updated_at` together indicate which blocks should be considered for finding accumulator updates.
