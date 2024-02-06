FROM rust:1.75

WORKDIR /app

COPY src ./src
COPY migrations ./migrations
COPY .env Cargo.lock Cargo.toml ./

RUN cargo build --release

ENTRYPOINT [ "./target/release/zero2prod" ]