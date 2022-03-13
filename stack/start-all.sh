#!/usr/bin/env bash

docker network create external-kafka-rolling

docker-compose -f kafka-stack-all.yml down -v
docker-compose -f kafka-stack-all.yml up -d
docker-compose -f ./redash/redash.yml up -d

echo "stack started"
