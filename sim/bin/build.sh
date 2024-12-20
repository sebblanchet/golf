#!/usr/bin/env bash
set -ex

# compile
cargo build --release --target wasm32-unknown-unknown --no-default-features

# build wasm
CRATE_NAME=$(grep name Cargo.toml | awk '{ print $3 }' | tr -d '"')
rm -rfv bin/wasm/target/*
wasm-bindgen --out-dir bin/wasm/target --target web target/wasm32-unknown-unknown/release/${CRATE_NAME}.wasm

# serve
#basic-http-server bin/wasm
