# blob

## Overview

Before diving further into Schemas in it is important to understand the way these are stored in the chain. Schemas are stored on chain as a Blob in the Blob Storage module. They are identified and retrieved by their unique blob id, a 32 byte long hex string. They are authored by a DID and have a max size of 8192 bytes. The chain is agnostic to the contents of blobs and thus to schemas. Blobs may be used to store types of data other than schemas.

## Runtime

```rust
parameter_types! {
    // 8KB
    pub const MaxBlobSize: u32 = 8192;
    pub const StorageWeight: Weight = Weight::from_ref_time(1100);
}

impl blob::Config for Runtime {
    type MaxBlobSize = MaxBlobSize;
    type StorageWeight = StorageWeight;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        /* snip */
        BlobStore: blob::{Pallet, Call, Storage},
        /* snip */
    }
);
```

## Call

### `new`

Create a new immutable blob.

---

## Query

### `blobs`

Store blobOwner and WrapperBytes of blob data

---
