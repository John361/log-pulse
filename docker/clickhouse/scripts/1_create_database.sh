#!/bin/bash
set -e

echo "Initializing database: ${CLICKHOUSE_DB}"

clickhouse-client --user "${CLICKHOUSE_USER}" --password "${CLICKHOUSE_PASSWORD}" --query "
    CREATE DATABASE IF NOT EXISTS ${CLICKHOUSE_DB};

    CREATE TABLE IF NOT EXISTS ${CLICKHOUSE_DB}.logs (
        timestamp DateTime64(3, 'UTC'),
        level Enum8('DEBUG'=0, 'INFO'=1, 'WARN'=2, 'ERROR'=3, 'CRITICAL'=4),
        service_name String,
        message String,
        metadata Map(String, String)
    )
    ENGINE = MergeTree()
    PARTITION BY toYYYYMM(timestamp)
    ORDER BY (service_name, timestamp);
"
