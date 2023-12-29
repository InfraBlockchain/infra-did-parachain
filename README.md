# Infra DID parachain

## 🚀 Getting Started

### 🦀 Rust Setup

Make sure you have Rust installed along with everything that's needed to compile a substrate node. More details [here](./docs/rust-setup.md).

### 🔧 Build

1. Clone the Infra DID parachain repository:

```sh
git clone https://github.com/InfraBlockchain/infra-did-parachain
```

2. Use `cargo` to build the parachain node without launching it:

```sh
cargo build --release
```

### 🕸️ Run a local network
 You will need a compatible release of [infrablockchain-substrate](https://github.com/InfraBlockchain/infrablockchain-substrate) to run a local network. You may also want to use [Zombienet](https://github.com/paritytech/zombienet/releases) (available for Linux and MacOS),  for spinning up a full fledged relay chain - parachain environment. You can find more information about running a local test network [HERE](./docs/zombienet.md)