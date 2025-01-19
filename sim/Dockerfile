# install deps
FROM rust:1.84 AS base
RUN cargo install -f wasm-bindgen-cli --version 0.2.100
RUN cargo install wasm-opt
RUN rustup target add wasm32-unknown-unknown

# build app
FROM base AS build
WORKDIR /out
WORKDIR /app
COPY . .
RUN ./bin/build.sh /out

# package
FROM alpine as pkg
WORKDIR /out
COPY --from=build /out .
RUN ls -alR /out
