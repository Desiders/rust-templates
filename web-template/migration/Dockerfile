# syntax=docker/dockerfile:1

FROM alpine:3.22 AS base
RUN apk add --no-cache ca-certificates && rm -rf /var/cache/apk/*

FROM rust:1.95 AS build
RUN apt-get update \
    && apt-get install -y --no-install-recommends musl-tools upx-ucl \
    && rm -rf /var/lib/apt/lists/*
RUN rustup target add x86_64-unknown-linux-musl

ADD --checksum=sha256:f2f6555dee50945c2a90a7fb793c2084a00cc6459d08ac2306d38d905064321b \
    https://github.com/kunobi-ninja/kache/releases/download/v1.2.2/kache-x86_64-unknown-linux-musl.tar.gz /tmp/kache.tar.gz
RUN tar -xzf /tmp/kache.tar.gz -C /usr/local/bin kache && rm /tmp/kache.tar.gz
ENV RUSTC_WRAPPER=/usr/local/bin/kache KACHE_CACHE_DIR=/root/.cache/kache

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY tg-bot-template/Cargo.toml tg-bot-template/Cargo.toml
COPY tg-bot-template/migration/Cargo.toml tg-bot-template/migration/Cargo.toml
COPY web-template/Cargo.toml web-template/Cargo.toml
COPY web-template/migration/Cargo.toml web-template/migration/Cargo.toml
RUN --mount=type=cache,id=web-template-migration-kache,target=/root/.cache/kache,sharing=locked \
    mkdir -p tg-bot-template/src tg-bot-template/migration/src web-template/src web-template/migration/src \
    && echo 'fn main() {}' > tg-bot-template/src/main.rs \
    && echo 'fn main() {}' > web-template/src/main.rs \
    && touch tg-bot-template/migration/src/lib.rs web-template/src/lib.rs \
    && echo 'fn main() {}' > web-template/migration/src/main.rs \
    && touch web-template/migration/src/lib.rs \
    && cargo build --locked --release --target x86_64-unknown-linux-musl -p web_template_migration \
    && rm -rf web-template/migration/src

COPY web-template/migration/src web-template/migration/src
RUN --mount=type=cache,id=web-template-migration-kache,target=/root/.cache/kache,sharing=locked \
    touch web-template/migration/src/main.rs web-template/migration/src/lib.rs \
    && cargo build --locked --release --target x86_64-unknown-linux-musl -p web_template_migration \
    && upx --best --lzma target/x86_64-unknown-linux-musl/release/web_template_migration

FROM base AS final
WORKDIR /app
COPY --from=build /usr/src/app/target/x86_64-unknown-linux-musl/release/web_template_migration /app/migration
ENV RUST_BACKTRACE=1
ENTRYPOINT ["/app/migration"]
