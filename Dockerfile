ARG RUST_VERSION=1.98.0

FROM rust:${RUST_VERSION} AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/nyxos /app/service

EXPOSE 3000

CMD ["./service", "start"]