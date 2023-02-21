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

<h3 align="center">Infra did substrate node</h3>

  <p align="center">
    An DID node using the Substrate Blockchain Framework.
    <br />
    <a href="https://github.com/InfraBlockchain/infra-did-substrate"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/InfraBlockchain/infra-did-substrate">View Demo</a>
    ·
    <a href="https://github.com/InfraBlockchain/infra-did-substrate/issues">Report Bug</a>
    ·
    <a href="https://github.com/InfraBlockchain/infra-did-substrate/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>

### Built With

- [Rust](https://www.rust-lang.org/)
- [Substrate](https://substrate.io/)
- [docker](https://www.docker.com/)

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- GETTING STARTED -->

## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Build locally

1. Clone the repo

   ```sh
   git clone https://github.com/InfraBlockchain/infra-did-substrate.git
   ```

2. Build
   ```sh
   cargo build --release
   ```

### How to run

1. Use Rust's native cargo command to build and launch the template node:

   ```sh
   cargo run --release -- --dev
   ```

### Single-Node Development Chain

This command will start the single-node development chain with non-persistent state:

```bash
./target/release/node-template --dev
```

Purge the development chain's state:

```bash
./target/release/node-template purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_BACKTRACE=1 ./target/release/node-template -ldebug --dev
```

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- CONTRIBUTING -->

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- CONTACT -->

## Contact

Project Link: [https://github.com/InfraBlockchain/infra-did-substrate](https://github.com/InfraBlockchain/infra-did-substrate)

<p align="right">(<a href="#top">back to top</a>)</p>
