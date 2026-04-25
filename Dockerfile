# Etapa 1: Compilación (Cambiamos a 1.78)
FROM rust:1.78-slim as builder

# Instalamos dependencias del sistema necesarias para compilar reqwest/openssl
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .
# Compilamos
RUN cargo build --release

# Etapa 2: Ejecución
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/apiRust /app/apiRust
RUN chmod +x /app/apiRust

EXPOSE 10000
ENV PORT=10000
CMD ["/app/apiRust"]