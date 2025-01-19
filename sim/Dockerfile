FROM rust:1.84

# init
ENV NAME ball
WORKDIR /out
WORKDIR /app
COPY . .

# install
RUN cargo install -f wasm-bindgen-cli --version 0.2.100
RUN cargo install wasm-opt
RUN cargo install basic-http-server
RUN rustup target add wasm32-unknown-unknown

# build
RUN ./bin/build.sh /out

# serve
WORKDIR /out
RUN ls -alR /out
ENTRYPOINT ["basic-http-server", "-a", "0.0.0.0:8004"]
