#!/usr/bin/env bash

docker network create external-kafka-rolling

docker-compose -f kafka-stack.yml down -v
docker-compose -f kafka-stack.yml up -d

echo "be patient"
sleep 15

NODE_TTL=60
echo "starting death loop, step :" $NODE_TTL "Second"


while true
do

docker-compose -f kafka-node-A.yml up -d
sleep $NODE_TTL
docker-compose -f kafka-node-B.yml down -v
sleep $NODE_TTL
docker-compose -f kafka-node-B.yml up -d
sleep $NODE_TTL
docker-compose -f kafka-node-A.yml down -v
sleep $NODE_TTL

done
