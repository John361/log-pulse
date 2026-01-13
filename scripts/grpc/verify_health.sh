#!/bin/bash

grpcurl -plaintext "[::1]:8080" grpc.health.v1.Health/Check
