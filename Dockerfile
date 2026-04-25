# Etapa 1: Compilación
FROM rust:1.75-slim as builder
WORKDIR /app
COPY . .
# Compilamos para release
RUN cargo build --release

# Etapa 2: Ejecución
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
# Copiamos el binario desde la etapa de compilación
COPY --from=builder /app/target/release/apiRust /app/apiRust
RUN chmod +x /app/apiRust
EXPOSE 10000
CMD ["/app/apiRust"]