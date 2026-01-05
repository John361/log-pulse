CREATE TABLE IF NOT EXISTS logs (
    timestamp DateTime64(3, 'UTC'),
    level Enum8('DEBUG'=0, 'INFO'=1, 'WARN'=2, 'ERROR'=3, 'CRITICAL'=4),
    service_name String,
    message String,
    metadata Map(String, String)
)
ENGINE = MergeTree()
PARTITION BY toYYYYMM(timestamp)
ORDER BY (service_name, timestamp);
