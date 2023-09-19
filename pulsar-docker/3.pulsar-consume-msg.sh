#!/usr/bin/env bash

docker exec -it hello_pulsar /pulsar/bin/pulsar-client consume a-topic -s "sub-a"
docker exec -it hello_pulsar /pulsar/bin/pulsar-client consume b-topic -s "sub-b"
