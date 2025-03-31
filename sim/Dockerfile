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

# package and serve
FROM busybox as pkg
WORKDIR /out
COPY --from=build /out .
ENTRYPOINT httpd -f -v -p 9002 -h .
