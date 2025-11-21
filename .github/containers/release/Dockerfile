FROM ubuntu:22.04 as base

ENV DEBIAN_FRONTEND=noninteractive

RUN apt update && \
    apt -y install \
        build-essential \
        cmake \
        curl \
        git \
        libssl-dev \
        ninja-build \
        pkg-config \
        python3 \
        zlib1g-dev

RUN curl -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
ENV RISC0_HOME="/risc0"

WORKDIR /src

FROM base AS builder
COPY . .

RUN ls -al rust/.git

WORKDIR /src/risc0
RUN --mount=type=cache,id=release,target=/root/.cache \
  --mount=type=cache,id=release,target=/root/.cargo,from=base,source=/root/.cargo \
  cargo build --bin rzup

RUN mkdir -p /risc0

RUN --mount=type=cache,id=release,target=/root/.cache \
  --mount=type=cache,id=release,target=/root/.cargo,from=base,source=/root/.cargo \
  target/debug/rzup build rust --path /src/rust

FROM scratch AS export
COPY --from=builder /risc0/toolchains/ /
