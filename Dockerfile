# ── Stage 1: compilar ──────────────────────────────────────
FROM rust:1.78-slim-bookworm AS builder

WORKDIR /app

# Dependencias del sistema para sqlx/sqlite
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

# Copiar manifiestos primero (cache de dependencias)
COPY Cargo.toml Cargo.lock ./

# Truco: compilar deps vacías primero para cachear
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copiar código real
COPY src ./src

# Build final (solo recompila src/)
RUN touch src/main.rs && cargo build --release

# ── Stage 2: imagen mínima ──────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libsqlite3-0 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/destajo .

# Directorio para la base de datos SQLite
RUN mkdir -p /data

ENV DATABASE_URL=sqlite:/data/destajo.db

EXPOSE 3000

CMD ["./destajo"]
