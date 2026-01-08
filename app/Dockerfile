# --- Builder ---
FROM rust:1.92.0-slim-trixie AS builder
WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY ingestor/Cargo.toml ./ingestor/

RUN mkdir -p ingestor/src/bin ingestor/src/lib && \
    echo "fn main() {}" > ingestor/src/bin/main.rs && \
    touch ingestor/src/lib/lib.rs
RUN cargo build --release --bin log-pulse-ingestor

COPY ingestor/src ./ingestor/src

RUN cargo build --release --bin log-pulse-ingestor


# --- Runtime ---
FROM debian:trixie-slim

WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates libssl3 gettext && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/log-pulse-ingestor /app/log-pulse-ingestor

EXPOSE 8080
CMD ["./log-pulse-ingestor"]
