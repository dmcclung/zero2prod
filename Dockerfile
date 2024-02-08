FROM rust:1.75-alpine AS builder

RUN apk add --no-cache musl-dev

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx
COPY .env Cargo.lock Cargo.toml ./

RUN SQLX_OFFLINE=true cargo build --target x86_64-unknown-linux-musl --release


FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/zero2prod ./zero2prod
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/.env ./.env

ENTRYPOINT [ "./zero2prod" ]