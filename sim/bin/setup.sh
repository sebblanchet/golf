#!/bin/env bash

set -ex

# Exit if rustup is not installed
if ! command -v rustup &>/dev/null; then
  echo "Error: rustup is not installed."
  exit 1
fi

# Exit if cargo is not installed
if ! command -v cargo &>/dev/null; then
  echo "Error: cargo is not installed."
  exit 1
fi

# Install wasm-bindgen-cli if not installed
if ! command -v wasm-bindgen &>/dev/null; then
  echo "Installing wasm-bindgen-cli"
  cargo install wasm-bindgen-cli
fi

# Install basic-http-server if not installed
if ! command -v basic-http-server &>/dev/null; then
  echo "Installing basic-http-server"
  cargo install basic-http-server
fi

# Create wasm folder if it doesn't already exist
mkdir -p assets

# Looks for `name = "crate_name"`, gets the third word (`"crate_name"`) and removes the quotes.
name=$(grep name Cargo.toml | awk '{ print $3 }' | tr -d '"')

# Create wasm/index.html if it doesn't already exist
echo "Creating file index.html'"
cat >assets/index.html <<EOF
<html>

<head>
    <meta charset="UTF-8" />
    <style>
        body {
            background: linear-gradient(135deg,
                    white 0%,
                    white 49%,
                    black 49%,
                    black 51%,
                    white 51%,
                    white 100%);
            background-repeat: repeat;
            background-size: 20px 20px;
        }

        canvas {
            background-color: white;
        }
    </style>
</head>
<script type="module">
    import init from './target/${name}.js'
    init()
</script>

</html>
EOF

echo "Adding rust cross-compilation target 'wasm32-unknown-unknown'"
rustup target add wasm32-unknown-unknown
