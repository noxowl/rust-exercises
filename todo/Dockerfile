FROM rust:1.64-alpine3.16 AS builder

RUN apk update
RUN apk upgrade
RUN apk add libc-dev=0.7.2-r3
RUN rm -rf /var/cache/apk/*

WORKDIR /todo
COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release
# for faster build.

COPY ./src ./src
COPY ./templates ./templates
RUN rm -f target/release/deps/todo*
RUN cargo build --release

FROM alpine:3.16

COPY --from=builder /todo/target/release/todo /usr/local/bin/todo
CMD ["todo"]