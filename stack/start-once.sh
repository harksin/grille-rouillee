#!/usr/bin/env bash

docker network create external-kafka-rolling

docker-compose -f kafka-stack.yml down -v
docker-compose -f kafka-stack.yml up -d

echo "stack started"
