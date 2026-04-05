# ===== Stage 1: フロントエンドビルド =====
FROM node:20-slim AS frontend-builder
WORKDIR /app/frontend

COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci

COPY frontend/ ./
RUN npm run build

# ===== Stage 2: バックエンドビルド =====
FROM rust:1.78-slim AS backend-builder
WORKDIR /app

COPY backend/Cargo.toml backend/Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs && cargo build --release && rm -rf src

COPY backend/src/ src/
RUN touch src/main.rs && cargo build --release

# ===== Stage 3: 実行環境 =====
FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -s /bin/bash appuser

WORKDIR /app

COPY --from=backend-builder /app/target/release/certmanager /app/certmanager
COPY --from=frontend-builder /app/frontend/dist /app/static

RUN chown -R appuser:appuser /app

USER appuser

EXPOSE 8080

ENV SERVER_PORT=8080
ENV RUST_LOG=info
ENV STATIC_DIR=/app/static

CMD ["/app/certmanager"]
