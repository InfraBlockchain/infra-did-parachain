# revoke

## Overview

Credential revocation is managed with on-chain revocation registries. To revoke a credential, its id (or hash of its id) must be
added to the credential. It is advised to have one revocation registry per credential type. Each registry has a unique id and
an associated policy. The policy determines who can update the revocation registry. The registry also has an "add-only" flag specifying
whether an id once added to the registry can be removed (leading to undoing the revocation) or not.
Similar to the replay protection mechanism for DIDs, for each registry, the last modified block number is kept which is updated
each time a credential is revoked or unrevoked.
For now, only one policy is supported which is that each registry is owned by a single DID. Also, neither the policy
nor the "add-only" flag can be updated post the creation of the registry for now.

## Runtime

```rust
parameter_types! {
    pub const MaxControllers: u32 = 15;
}

impl revoke::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxControllers = MaxControllers;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        /* snip */
        Revoke: revoke::{Pallet, Call, Storage, Event},
        /* snip */
    }
);
```

## Call

### `newRegistry(add_registry)`

Create a new revocation registry named `id` with `registry` metadata.

Returns an error if `id` is already in use as a registry id.

Returns an error if `registry.policy` is invalid.

---

### `removeRegistry(removal,proof)`

Delete an entire registry. Deletes all revocations within the registry, as well as registry metadata. Once the registry is deleted, it can be reclaimed by any party using a call to `new_registry`.

Returns an error if the registry referenced by `revoke.registry_id` is `add_only`.

Returns an error if `removal.last_modified` does not match the block number when the registry referenced by `removal.registry_id` was last modified.

Returns an error if `proof` does not satisfy the policy requirements of the registry referenced by `removal.registry_id`.

---

### `revoke(revoke,proof)`

Create some revocations according to the `revoke` command.

Returns an error if `revoke.last_modified` does not match the block number when the registry referenced by `revoke.registry_id` was last modified.

Returns an error if `proof` does not satisfy the policy requirements of the registry referenced by `revoke.registry_id`.

---

### `unrevoke(unrevoke,proof)`

Delete some revocations according to the `unrevoke` command.

Returns an error if the registry referenced by `revoke.registry_id` is `add_only`.

Returns an error if `unrevoke.last_modified` does not match the block number when the registry referenced by `revoke.registry_id` was last modified.

Returns an error if `proof` does not satisfy the policy requirements of the registry referenced by `unrevoke.registry_id`.

---

## Query

### `registries`

RegistryId -> Registry

Registry metadata

---

### `revocations`

(RegistryId, RevokeId) -> ()

The single global revocation set

---
