ARG RUST_VERSION=1.98.0

FROM rust:${RUST_VERSION} AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY crates/ crates/

RUN cargo build --locked --release --package nyxos

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/nyxos /app/nyxos

EXPOSE 3000

ENTRYPOINT [ "/app/nyxos" ]

CMD ["start"]
