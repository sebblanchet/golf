#!/usr/bin/env bash

set -ex

out=${1:-wasm}
dst="$out/target"

name=$(grep name Cargo.toml | awk '{ print $3 }' | tr -d '"')

# compile
cargo build --profile=wasm-release --target wasm32-unknown-unknown --no-default-features

# build wasm
mkdir -p $dst
rm -rfv $dst/*
wasm-bindgen target/wasm32-unknown-unknown/wasm-release/${name}.wasm --out-dir $dst --no-modules --no-typescript
du -h "$dst/${name}_bg.wasm"

# optimize
wasm-opt -Oz "$dst/${name}_bg.wasm" -o "$dst/${name}_opt.wasm"
du -h "$dst/${name}_opt.wasm"

# copy index
cp -rfv assets/index.html $out
ls -alR $out
