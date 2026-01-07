#!/bin/bash

grpcurl -plaintext \
    -import-path /home/manjaro/Documents/gitspace/log-pulse/app/ingestor/src/proto \
    -proto log.proto \
    -d '{
        "service_name": "test-cli",
        "message": "Hello LogPulse from grpcurl!",
        "level": "INFO",
        "timestamp": "2026-01-05T10:00:00Z"
    }' \
    [::1]:8080 \
    grpc.log.LogIngestorGrpc/SendLog
