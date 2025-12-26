# Stage 1: Build
FROM rust:1.75-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

RUN cargo build --release --package axum-sea-forms-cli

# Stage 2: Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/asf /usr/local/bin/asf

# Create data directory for SQLite databases
RUN mkdir -p /data
WORKDIR /data

EXPOSE 3000

CMD ["asf", "serve"]
