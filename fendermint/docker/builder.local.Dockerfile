# syntax=docker/dockerfile:1

# Builder
FROM rust:bookworm as builder

RUN apt-get update && \
  apt-get install -y build-essential clang cmake protobuf-compiler && \
  rm -rf /var/lib/apt/lists/*

WORKDIR /workdir

RUN ls -al /workdir
RUN cargo build -vvv --locked -p fendermint_app
RUN cargo build -vvv --locked --path /src/ipc/cli
