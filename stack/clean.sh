#!/usr/bin/env bash


docker-compose -f kafka-stack.yml down -v

docker-compose -f kafka-node-B.yml down -v

docker-compose -f kafka-node-A.yml down -v

docker system prune
