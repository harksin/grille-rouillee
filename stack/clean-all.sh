#!/usr/bin/env bash


docker-compose -f kafka-stack-all.yml down -v

docker-compose -f ./redash/redash.yml up -d


