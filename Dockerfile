FROM ubuntu:jammy AS builder

# The node will be built in this directory
WORKDIR /infra-did-parachain

RUN apt -y update && \
  apt install -y --no-install-recommends \
  software-properties-common llvm curl git file binutils binutils-dev \
  make cmake ca-certificates clang g++ zip dpkg-dev openssl gettext \
  build-essential pkg-config libssl-dev libudev-dev time clang protobuf-compiler

# install rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# rustup directory
ENV PATH /root/.cargo/bin:$PATH

# setup rust nightly channel, pinning specific version as newer versions have a regression
RUN rustup install nightly-2023-12-01

RUN rustup default nightly-2023-12-01

# install wasm toolchain for substrate
RUN rustup target add wasm32-unknown-unknown --toolchain nightly-2023-12-01

#compiler ENV
ENV CC clang
ENV CXX g++

# Copy code to build directory, instead of only using .dockerignore, we copy elements
# explicitly. This lets us cache build results while iterating on scripts.
COPY . .

RUN echo 'Building in release mode.' ; \
    cargo build --release ; \
    mv /infra-did-parachain/target/release/infra-did-parachain /infra-did-parachain/target/; 

# Final stage. Copy the node executable and the script
FROM ubuntu:jammy

WORKDIR /infra-did-parachain

COPY --from=builder /infra-did-parachain/target/infra-did-parachain .

# curl is required for uploading to keystore
# note: `subkey insert` is a potential alternarve to curl
RUN apt -y update \
  && apt install -y --no-install-recommends curl \
  && rm -rf /var/lib/apt/lists/*

# expose node ports
EXPOSE 30333 9933 9944

ENV RUST_BACKTRACE 1

ENTRYPOINT ["./infra-did-parachain"]
CMD []