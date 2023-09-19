#!/usr/bin/env bash
export GO111MODULE=on
export GOPROXY=https://goproxy.cn
go get hello-pulsar
go install