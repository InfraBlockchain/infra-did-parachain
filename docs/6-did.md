# did

## Runtime

```rust
parameter_types! {
    pub const MaxDidDocRefSize: u16 = 1024;
    pub const DidDocRefPerByteWeight: Weight = Weight::from_ref_time(10);
    pub const MaxServiceEndpointIdSize: u16 = 1024;
    pub const ServiceEndpointIdPerByteWeight: Weight = Weight::from_ref_time(10);
    pub const MaxServiceEndpointOrigins: u16 = 64;
    pub const MaxServiceEndpointOriginSize: u16 = 1025;
    pub const ServiceEndpointOriginPerByteWeight: Weight = Weight::from_ref_time(10);
}

impl did::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxDidDocRefSize = MaxDidDocRefSize;
    type DidDocRefPerByteWeight = DidDocRefPerByteWeight;
    type MaxServiceEndpointIdSize = MaxServiceEndpointIdSize;
    type ServiceEndpointIdPerByteWeight = ServiceEndpointIdPerByteWeight;
    type MaxServiceEndpointOrigins = MaxServiceEndpointOrigins;
    type MaxServiceEndpointOriginSize = MaxServiceEndpointOriginSize;
    type ServiceEndpointOriginPerByteWeight = ServiceEndpointOriginPerByteWeight;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        /* snip */
        DIDModule: did::{Pallet, Call, Storage, Event, Config},
        /* snip */

    }
);
```

## Call

### `addControllers`

Add new controllers.

**Does not** require provided controllers to

- have any key
- exist on- or off-chain

---

### `addKeys`

Add more keys from DID doc.

**Does not** check if the key was already added.

---

### `addServiceEndpoint`

Add a single service endpoint.

---

### `newOffchain`

Add did from offchain document

document can be provided URL, CID, Bytes

---

### `newOnchain`

Create new DID.

At least 1 control key or 1 controller must be provided.

If any supplied key has an empty `ver_rel`, then it will use all verification relationships available for its key type.

---

### `noop`

Adds `StateChange` to the metadata.

---

### `removeControllers`

Remove controllers.

This is an atomic operation meaning that it will either remove all keys or do nothing.

**Note that removing all might make DID unusable**.

---

### `removeKeys`

Remove keys from DID doc. This is an atomic operation meaning that it will either remove all keys or do nothing.

**Note that removing all might make DID unusable**

---

### `removeOffchainDid`

Remove did added from `newOffchain` extrinsic operation

---

### `removeOnchainDid`

Remove the on-chain DID along with its keys, controllers, service endpoints and BBS+ keys.

Other DID-controlled entities won't be removed.

However, the authorization logic ensures that once a DID is removed, it loses its ability to control any DID.

---

### `removeServiceEndpoint`

Remove a single service endpoint.

---

### `setOffchainDidDocRef`

Update offchain did added from `newOffchain` extrinsic operation

document can be provided URL, CID, Bytes

---

## Query

### `didControllers`

Stores controlled - controller pairs of a DID as (DID, DID) -> zero-sized record. If a record exists, then the controller is bound.

---

### `didKeys`

Stores keys of a DID as (DID, IncId) -> DidKey. Does not check if the same key is being added multiple times to the same DID.

---

### `didServiceEndpoints`

Stores service endpoints of a DID as (DID, endpoint id) -> ServiceEndpoint.

---

### `dids`

Stores details of off-chain and on-chain DIDs

---
