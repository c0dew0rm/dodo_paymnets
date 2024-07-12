FROM rust:latest

WORKDIR /usr/src/dodo_payments

COPY . .

RUN apt-get update && apt-get install -y libpq-dev && \
    cargo install diesel_cli --no-default-features --features postgres && \
    cargo build --release

CMD ["./target/release/dodo_payments"]
