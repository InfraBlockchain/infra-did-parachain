# master

## Call

### `execute(proposal, auth)`

Execute a proposal that has received enough votes. The proposal is a serialized Call.

This function can be called by anyone, even someone who is not a member of Master.

After a successful execution, the round number is increased.

---

### `executeUncheckedWeight(proposal, auth, weight)`

Does the same job as `execute` dispatchable but does not inherit the weight of the `Call` its wrapping but expects the caller to provide it and removes the votes for the previous round.

Since as a group members of master have root access, they will be able to call this function.

A vote requirement of zero is not allowed and will result in an error.

A vote requirement larger than the size of the member list is not allowed and will result in an error.

---

### `setMembers(membership)`

Root-only. Sets the members and vote requirement for master. Increases the round number

---

## Query

### `members`

Membership storage to store members and vote requirement thresholds

---

### `round`

u64
