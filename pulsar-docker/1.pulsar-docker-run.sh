#!/usr/bin/env bash
#
# https://hub.docker.com/r/apachepulsar/pulsar
export pulsar_version=3.1.0
#
docker run --rm --name hello_pulsar -it \
    -p 6650:6650 \
    -p 8080:8080 \
    --mount source=pulsardata,target=/pulsar/data \
    --mount source=pulsarconf,target=/pulsar/conf \
    apachepulsar/pulsar:${pulsar_version} \
    bin/pulsar standalone
