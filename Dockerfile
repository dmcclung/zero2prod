FROM rust:1.75 AS builder

WORKDIR /app

COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx
COPY .env Cargo.lock Cargo.toml ./

RUN SQLX_OFFLINE=true cargo build --release


FROM rust:1.75

WORKDIR /app

COPY --from=builder /app/target/release/zero2prod ./zero2prod
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/.env ./.env

ENTRYPOINT [ "./zero2prod" ]