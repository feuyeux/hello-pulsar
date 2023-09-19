#!/usr/bin/env bash

curl -s http://localhost:8080/admin/v2/persistent/public/default/a-topic/stats | jq
# docker exec -it hello_pulsar /pulsar/bin/pulsar-admin topics list public/default
