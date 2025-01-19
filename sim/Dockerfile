FROM rust:1.84

ENV NAME ball

# init
WORKDIR /app
COPY . .

# build
RUN cargo install wasm-bindgen-cli
RUN cargo install wasm-opt
RUN rustup target add wasm32-unknown-unknown
RUN cargo build --profile=wasm-release --target wasm32-unknown-unknown --no-default-features
RUN wasm-bindgen --out-dir . --target web target/wasm32-unknown-unknown/release/${NAME}.wasm
RUN wasm-opt -Oz "${NAME}_bg.wasm" -o "${NAME}_opt.wasm"
