# anchor

## Overview

Anchors are hashed once before being added to storage. To check whether an anchor exists query the "Anchors" map for the hash of the anchor. If a corresponding value exists, then the anchor exists and the value represents the block number when it was first published.

---

## Call

### `deploy`

Drop a permanent anchor.

---

## Query

### `anchors`

Hasher can be the identity here becuse we perform a hash ourself which has the same merkle-trie balancing effect as using a hash-prefix map.

---
