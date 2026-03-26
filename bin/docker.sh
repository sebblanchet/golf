#!/usr/bin/env bash

set -e

name="golf-sim"
verbose=0
deploy=0
stop=0
tmpdir=$(mktemp -d)

function has() {
  if type -p "$1" >/dev/null; then
    echo "has $1"
    return 0
  fi

  echo "install $1"
  return 1
}

function run_checks() {
  has docker
  has yq
}

function run_stop() {
  echo "stopping running image"
  docker stop "$name" || true
  docker rm "$name" || true
}

function run_build() {
  echo "building image"

  # build and start container
  docker build -t "$name" .
  docker run -d --name "$name" -p 9002:9002 "$name"

  # extract files for container
  docker cp "$name":/out/ $tmpdir
}

function run_deploy() {
  echo "deploy"
  ver="v$(yq e '.package.version' Cargo.toml)"
  git checkout gh-pages
  cp -rfv $tmpdir/out/* .
  git add --all
  git commit -m "deploy $ver"
  git tag $ver
}

while [[ $# -gt 0 ]]; do
  case $1 in
  -d | --deploy)
    deploy=1
    shift
    ;;
  -s | --stop)
    stop=1
    shift
    ;;
  -v | --verbose)
    verbose=1
    shift
    ;;
  -h | --help)
    echo "$0 [options]"
    echo "-h,--help: print this usage"
    echo "-d,--deploy: deploy the output to github pages"
    echo "-s,--stop: stop the docker image after extracting build files"
    echo "-v,--verbose:  set -x"
    exit 0
    ;;
  *)
    echo "unknown option $1"
    exit 1
    ;;
  esac
done

if ((verbose)); then
  set -x
fi

run_checks
run_stop
run_build

if ((stop)); then
  run_stop
fi

if ((deploy)); then
  run_deploy
fi
