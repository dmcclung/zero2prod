FROM rust:1.75

WORKDIR /app

COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx
COPY .env Cargo.lock Cargo.toml ./

RUN SQLX_OFFLINE=true cargo build --release

ENTRYPOINT [ "./target/release/zero2prod" ]