# revoke

## Call

### `addIssuer(entity, proof)`

Create some issuer according to the `entity` command.

Returns an error if `entity.last_modified` does not match the block number when the authorizer referenced by `entity.authorizer_id` was last modified.

Returns an error if `proof` does not satisfy the policy requirements of the authorizer referenced by `entity.authorizer_id`.

---

### `addVerifier(entity, proof)`

Create some verifier according to the `entity` command.

Returns an error if `entity.last_modified` does not match the block number when the authorizer referenced by `entity.authorizer_id` was last modified.

Returns an error if `proof` does not satisfy the policy requirements of the authorizer referenced by `entity.authorizer_id`.

---

### `newAuthorizer(addAuthorizer)`

Create a new authrorizer named `id` with `authrorizer` metadata.

Returns an error if `id` is already in use as a authrorizer id.

Returns an error if `authrorizer.policy` is invalid.

---

### `removeAuthorizer(removal, proof)`

Delete an entire authorizer. Deletes all trusted entities within the authorizer, as well as authorizer metadata. Once the authorizer is deleted, it can be reclaimed by any party using a call to `new_authorizer`.

Returns an error if the authorizer referenced by `entity.authorizer_id` is `add_only`.

Returns an error if `removal.last_modified` does not match the block number when the authorizer referenced by `removal.authorizer_id` was last modified.

Returns an error if `proof` does not satisfy the policy requirements of the authorizer referenced by `removal.authorizer_id`.

---

### `removeIssuer(entity, proof)`

Delete some issuer according to the `entity` command.

Returns an error if the authorizer referenced by `entity.authorizer_id` is `add_only`.

Returns an error if `entity.last_modified` does not match the block number when the authorizer referenced by `authrorizer.authorizer_id` was last modified.

Returns an error if `proof` does not satisfy the policy requirements of the authorizer referenced by `entity.authorizer_id`.

---

### `removeVerifier(entity, proof)`

Delete some verifier according to the `entity` command.

Returns an error if the authorizer referenced by `entity.authorizer_id` is `add_only`.

Returns an error if `entity.last_modified` does not match the block number when the authorizer referenced by `authrorizer.authorizer_id` was last modified.

Returns an error if `proof` does not satisfy the policy requirements of the authorizer referenced by `entity.authorizer_id`.

---

## Query

### `Authorizers`

AuthorizerId -> Authorizer

Authorizer metadata

---

### `Issuers`

(AuthorizerId, TrustedEntityId) -> ()

The single global issuers set

---

### `Verifiers`

(AuthorizerId, TrustedEntityId) -> ()

The single global verifiers set

---
