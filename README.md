# Log Pulse

A high-performance log ingestion service built in Rust. It acts as a reliable buffer layer between distributed applications and ClickHouse, using Redis as a persistent staging area to ensure zero data loss and optimal write throughput.

## Features
- Asynchronous Ingestion: Utilizes Tokio and mpsc channels for non-blocking log collection
- Redis Buffering: Protects against traffic spikes and ClickHouse downtime by staging logs in Redis lists
- Batch Processing: Optimized ClickHouse inserts by grouping logs into configurable batches
- Security First:
  - Memory safety via Zeroize to clear secrets from RAM
  - Log-safe secrets (custom Debug implementation to prevent password leaks)
- Resilient Design: Multiplexed Redis connections and robust error handling in the background worker

## Architecture
- Ingestion: Receives logs via a background task
- Staging: Immediately serializes and pushes logs to a Redis list (RPUSH)
- Flushing: A periodic worker pops batches from Redis and performs a bulk insert into ClickHouse

## Configuration & Startup
### Docker
```shell
cd docker

cp .env.template .env
# Replace "changeme" values

docker-compose -f docker/docker-compose.yml up --build
```

### Application
```shell
cd app/ingestor

cp app.conf.template.yml app.conf.yml
# Replace "changeme" values with those from .env

RUST_LOG=debug cargo run
```
