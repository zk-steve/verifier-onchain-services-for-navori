# Using the `rust-musl-builder` as base image, instead of
# the official Rust toolchain
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN apt update
RUN apt install -y git build-essential libssl-dev pkg-config protobuf-compiler libclang1 clang \
    cmake \
    libpq-dev \
    libdw-dev \
    binutils \
    lld \
    libudev-dev
RUN rm -rf /var/lib/apt/lists/*

RUN git clone https://github.com/Draply/source.git /root/.stone-cli/v0.1.0
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --all
RUN mv target/${CARGO_BUILD_TARGET}/release /out

FROM debian:bookworm-slim AS irelia-public-server
WORKDIR /user
RUN apt update
RUN apt install -y libssl-dev libpq-dev
RUN rm -rf /var/lib/apt/lists/*
COPY crates/public/config/00-default.toml 00-default.toml
COPY --from=builder /out/irelia /usr/local/bin/irelia
ENTRYPOINT ["/usr/local/bin/irelia", "--config-path=*.toml"]

FROM debian:bookworm-slim AS irelia-worker
WORKDIR /user
RUN apt update
RUN apt install -y libssl-dev libpq-dev
RUN rm -rf /var/lib/apt/lists/*
COPY crates/worker/config/00-default.toml 00-default.toml
COPY --from=builder /out/irelia_worker /usr/local/bin/irelia_worker
ENTRYPOINT ["/usr/local/bin/irelia_worker", "--config-path=*.toml"]
