<div id="top"></div>
<p align="center">
<img src=https://img.shields.io/github/stars/InfraBlockchain/infra-did-substrate?style=for-the-badge&logo=appveyor&color=blue />
<img src=https://img.shields.io/github/forks/InfraBlockchain/infra-did-substrate?style=for-the-badge&logo=appveyor&color=blue />
<img src=https://img.shields.io/github/issues/InfraBlockchain/infra-did-substrate?style=for-the-badge&logo=appveyor&color=informational />
<img src=https://img.shields.io/github/issues-pr/InfraBlockchain/infra-did-substrate?style=for-the-badge&logo=appveyor&color=informational />
</p>
<br />
<!-- PROJECT LOGO -->
<p align="center">
  <a href="https://substrate.io/" target="blank"><img src="https://cdn-images-1.medium.com/max/960/1*OQP5QAtLtrVCtNCKwB6GkQ.png" width="320" alt="Nest Logo" /></a>
</p>

<br />
<div align="center">
  <a href="https://github.com/InfraBlockchain/infra-did-substrate">
    <!-- <img src="images/logo.png" alt="Logo" width="80" height="80"> -->
  </a>

<h3 align="center">Infra DID Substrate node</h3>

  <p align="center">
    An Infra DID node using the Substrate Blockchain Framework.
    <br />
    <a href="https://github.com/InfraBlockchain/infra-did-substrate/tree/develop/docs"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Finfra2.infrablockchain.com#/explorer">View Demo</a>
    ·
    <a href="https://github.com/InfraBlockchain/infra-did-substrate/issues">Report Bug</a>
    ·
    <a href="https://github.com/InfraBlockchain/infra-did-substrate/issues">Request Feature</a>
  </p>
</div>

<!-- TOC -->

<h3 align="center">Table of Contents</h3>
  <p align="center">
    <a href="#1-introduction">1. Introduction</a><br>
    <a href="#2-overview">2. Overview</a><br>
    <a href="#3-building">3. Building</a><br>
    <a href="#4-run">4. Run</a><br>
    <a href="#5-development">5. Development</a><br>
  </p>

<!-- /TOC -->

## 1. Introduction

infra did substrate node is a type of DID (Decentralized Identifier) node that is based on a substrate framework. infra did substrate node are specifically designed to provide a secure and scalable infrastructure for the creation, management, and resolution of DIDs. They are built on top of a substrate framework, which is a modular framework for building blockchain-based applications.

<p align="right">(<a href="#top">back to top</a>)</p>

## 2. Overview

infra did substrate node is implemented by following the [infra-did spec](https://github.com/InfraBlockchain/infra-did-method-specs/blob/main/docs/Infra-DID-method-spec.md)

infra did substrate node contains pallets that allow you to creation, management the DID.

For more information, see the [docs](https://github.com/InfraBlockchain/infra-did-substrate/tree/develop/docs)

<p align="right">(<a href="#top">back to top</a>)</p>

## 3. Building

Install Rust:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

and install rust nightly:

```
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

for more information, see the [docs](https://github.com/InfraBlockchain/infra-did-substrate/tree/develop/docs/0-rust-setup.md)

and install other dependencies like profobuf, llvm:

```sh
# MacOS
brew install llvm protobuf
# Ubuntu
sudo apt-get install -y protobuf-compiler llvm
```

build infra-did node:

```sh
cargo build --release
```

<p align="right">(<a href="#top">back to top</a>)</p>

## 4. Run

You can start local development chain

```sh
./target/release/infradid --dev --alice --tmp
```

or start local chain

```sh
./target/release/infradid --chain=local --alice --tmp
```

<p align="right">(<a href="#top">back to top</a>)</p>

## 5. Development

You can start local development parachain with relay chain using [zombienet](https://github.com/paritytech/zombienet)

```sh
zombienet spawn --provider native zombienet/local-dev.toml
```

And apply rust formatter

```sh
cargo +nightly fmt
```
