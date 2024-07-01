FROM rust:1.79 as builder

RUN cargo install sqlx-cli --features postgres

WORKDIR /app
COPY . .
RUN cargo build --release

### ================================= ###

FROM debian:12-slim

RUN apt-get update && apt-get install -y \
    libgcc1 \
    libstdc++6 \
    openssl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY --from=builder /app/target/release/rust-app .
COPY --from=builder /app/migrations migrations

CMD ["bash", "-c", "sqlx migrate run && ./rust-app"]
