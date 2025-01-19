#!/usr/bin/env bash
set -ex

dst=bin/wasm/target
name=$(grep name Cargo.toml | awk '{ print $3 }' | tr -d '"')

# compile
cargo build --profile=wasm-release --target wasm32-unknown-unknown --no-default-features

# build wasm
rm -rfv $dst/*
wasm-bindgen --out-dir $dst --target web target/wasm32-unknown-unknown/release/${name}.wasm
du -h "$dst/${name}_bg.wasm"

# optimize
wasm-opt -Oz "$dst/${name}_bg.wasm" -o "$dst/${name}_opt.wasm"
du -h "$dst/${name}_opt.wasm"

# compress
gzip -9 <"$dst/${name}_opt.wasm" >"$dst/${name}_zip.wasm"
du -h "$dst/${name}_zip.wasm"
