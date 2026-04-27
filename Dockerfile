# Multi-stage Dockerfile para rust_chess
# Etapa 1: Compilar el motor Rust
FROM rust:1.85-bookworm AS rust-builder

WORKDIR /build

# Instalar dependencias de build necesarias
RUN apt-get update && \
    apt-get install -y --no-install-recommends pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copiar manifests primero para cachear dependencias
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Compilar en release
RUN cargo build --release

# Etapa 2: Compilar el servidor Go
FROM golang:1.23-bookworm AS go-builder

WORKDIR /build

# Copiar dependencias Go primero para cachear
COPY web/go.mod web/go.sum ./
RUN go mod download

# Copiar código fuente Go
COPY web/ ./

# Copiar el motor Rust compilado
COPY --from=rust-builder /build/target/release/rust_chess /tmp/rust_chess

# Compilar servidor Go
RUN CGO_ENABLED=0 GOOS=linux go build -ldflags="-w -s" -o server server.go

# Etapa 3: Imagen final mínima
FROM debian:bookworm-slim

WORKDIR /app

# Instalar solo lo necesario (git para benchmark, wget para healthcheck)
RUN apt-get update && \
    apt-get install -y --no-install-recommends git ca-certificates wget && \
    rm -rf /var/lib/apt/lists/*

# Crear directorio para el motor
RUN mkdir -p /app/target/release

# Copiar binarios desde las etapas anteriores
COPY --from=go-builder /build/server ./server
COPY --from=go-builder /tmp/rust_chess ./target/release/rust_chess
COPY --from=go-builder /build/static ./static

# Variables de entorno por defecto
ENV PORT=8080
ENV RUST_ENGINE_PATH=/app/target/release/rust_chess

# Exponer puerto
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/ || exit 1

# Ejecutar servidor
CMD ["./server"]
