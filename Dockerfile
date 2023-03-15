FROM ubuntu:jammy AS builder

# The node will be built in this directory
WORKDIR /parachain-template-node

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
RUN rustup install nightly

# install wasm toolchain for substrate
RUN rustup target add wasm32-unknown-unknown --toolchain nightly

#compiler ENV
ENV CC clang
ENV CXX g++

# Copy code to build directory, instead of only using .dockerignore, we copy elements
# explicitly. This lets us cache build results while iterating on scripts.
COPY . .

# Pass the features while building image as `--build-arg features='--features mainnet'` or `--build-arg features='--features testnet'`
ARG features
ARG release

RUN if [ "$release" = "Y" ] ; then \
      echo 'Building in release mode.' ; \
      WASM_BUILD_TOOLCHAIN=nightly cargo build --profile=release $features ; \
      mv /parachain-template-node/target/release/parachain-template-node /parachain-template-node/target/; \
    else \
      echo 'Building in production mode.' ; \
      WASM_BUILD_TOOLCHAIN=nightly cargo build --profile=production $features ; \
      mv /parachain-template-node/target/production/parachain-template-node /parachain-template-node/target/; \
    fi

# Final stage. Copy the node executable and the script
FROM ubuntu:jammy

WORKDIR /parachain-template-node

COPY --from=builder /parachain-template-node/target/parachain-template-node .

# curl is required for uploading to keystore
# note: `subkey insert` is a potential alternarve to curl
RUN apt -y update \
  && apt install -y --no-install-recommends curl \
  && rm -rf /var/lib/apt/lists/*

# expose node ports
EXPOSE 30333 9933 9944

ENV RUST_BACKTRACE 1

ENTRYPOINT ["./parachain-template-node"]
CMD []