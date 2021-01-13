FROM rust:1.48.0-alpine

RUN apk add protobuf protobuf-dev --repository=http://dl-cdn.alpinelinux.org/alpine/edge/main
RUN rustup component add rustfmt clippy