#!/usr/bin/env bash

set -ex

# build and start container
docker build -t golf-sim .
id=$(docker run -d --name golf-sim -it -v wasm:/out golf-sim sh)

# extract files for container
docker cp golf-sim:/out /tmp
docker kill $id
