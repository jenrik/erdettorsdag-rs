FROM rust:1.52-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /usr/src/erdettorsdag
COPY Cargo.lock Cargo.toml ./
COPY templates ./templates/
COPY src ./src/
RUN cargo build --release


FROM alpine

WORKDIR /

COPY --from=builder /usr/src/erdettorsdag/target/release/erdettorsdag .
COPY imgs ./imgs/
EXPOSE 8080
ENTRYPOINT ["/erdettorsdag"]