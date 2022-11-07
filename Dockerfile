# syntax=docker/dockerfile:1

FROM rust:1.65.0-slim-buster as builder
WORKDIR /update-fetch/

COPY Cargo.lock .
COPY Cargo.toml .
COPY default.nix .
COPY Dockerfile .
COPY src src

RUN cargo build --release

FROM scratch
COPY --from=builder /update-fetch/target/release/update-fetch .