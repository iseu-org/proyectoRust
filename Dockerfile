# Usamos una imagen de Linux mínima para que sea rápido y ligero
FROM alpine:latest

# Instalamos librerías básicas por si acaso (aunque tu binario sea estático)
RUN apk add --no-cache ca-certificates

# Copiamos TU binario ya compilado desde tu carpeta target a la imagen
COPY target/x86_64-unknown-linux-musl/release/apiRust /app/apiRust

# Le damos permisos de ejecución
RUN chmod +x /app/apiRust

# Exponemos el puerto que usa tu API (ejemplo: 8080)
EXPOSE 10000

# Comando para iniciar la API
CMD ["/app/apiRust"]