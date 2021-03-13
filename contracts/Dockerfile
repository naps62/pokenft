FROM rust:1.50-alpine3.13

WORKDIR /app

RUN apk add --no-cache musl-dev

RUN rustup default nightly
RUN cargo install parity-wasm --version 0.32.0
RUN cargo install substrate-wasm-builder --version v2.0.1
RUN cargo install wasm-builder-runner-impl --version v1.0.0

RUN cargo install canvas-node --git https://github.com/paritytech/canvas-node.git --tag v0.1.4 --force --locked

CMD ["canvas", "--dev", "--tmp"]
