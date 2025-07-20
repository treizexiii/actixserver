FROM rust:latest AS builder
WORKDIR /usr/src/actixserver

COPY . .
RUN cargo build --release

FROM debian:latest
COPY --from=builder /usr/src/actixserver/target/release/actixserver /usr/local/bin/actixserver
ENTRYPOINT ["actixserver"]
