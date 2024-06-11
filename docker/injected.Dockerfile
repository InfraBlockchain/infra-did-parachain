FROM docker.io/library/ubuntu:22.04

COPY ./target/release/infra-did-parachain /usr/local/bin

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/infra-did-parachain"]