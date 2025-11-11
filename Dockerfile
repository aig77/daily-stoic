# ===== BUILD STAGE =====
FROM rust:slim-trixie AS builder

RUN apt-get update && apt-get install -y \
  pkg-config \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# ===== RUNTIME STAGE =====
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
  ca-certificates \
  curl \
  && rm -rf /var/lib/apt/lists/*

RUN useradd -r -s /bin/false stoic

WORKDIR /app

COPY --from=builder /app/target/release/daily-stoic-api-rs /app/daily-stoic-api-rs

RUN chown -R stoic:stoic /app

USER stoic

EXPOSE 3000

ENV ADDRESS=0.0.0.0:3000
ENV DATABASE_PATH=/app/data/database.json

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/ || exit 1

CMD ["./daily-stoic-api-rs"]


