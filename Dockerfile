# Build Stage

FROM rust:bookworm AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev  \
    libclang-dev libssl-dev openssl

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app

COPY . .
COPY Cargo.toml Cargo.lock ./

# Source code and migrations
COPY src src
COPY migrations migrations
COPY i18n i18n

RUN cargo build --release

# Final Stage
FROM debian:bookworm-slim

# install dependencies
RUN apt-get update && apt-get install -y \
    libclang-dev \
    libpq-dev \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Add 'rusty' user group we want container to run as
RUN groupadd -r rusty && useradd --no-log-init -r -g rusty rusty

WORKDIR /app

# --chown=<user>:<group>
COPY --from=builder /app/target/release/earthdawn_creatures .

COPY templates templates
COPY static static
COPY i18n i18n

USER rusty
EXPOSE 8080

CMD ["./earthdawn_creatures"]