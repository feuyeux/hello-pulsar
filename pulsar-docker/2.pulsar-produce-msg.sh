#!/usr/bin/env bash

docker exec -it hello_pulsar /pulsar/bin/pulsar-client produce a-topic --messages "hello world"
# docker exec -it hello_pulsar /pulsar/bin/pulsar-client produce b-topic --messages "hello world"
