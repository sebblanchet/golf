#!/usr/bin/env bash

set -ex

name="golf-sim"

function stop() {
  docker stop "$name" || true && docker rm "$name" || true
}

function build() {
  stop

  # build and start container
  docker build -t "$name" .
  docker run -d --name "$name" -p 9002:9002 "$name"

  # extract files for container
  docker cp "$name":/out/ out
}

build
