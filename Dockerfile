# Etapa 1: Construcción
FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

# Instalar dependencias para compilar correctamente con OpenSSL
RUN apt-get update && apt-get install -y pkg-config libssl-dev && cargo build --release

# Etapa 2: Imagen final con dependencias mínimas
FROM debian:latest
WORKDIR /usr/src/app

# Instalar OpenSSL en la imagen final
RUN apt-get update && apt-get install -y libssl3 ca-certificates

# Copiar el binario desde la etapa de construcción
COPY --from=builder /usr/src/app/target/release/actix_web_api .

# Exponer el puerto y ejecutar
EXPOSE 8080
CMD ["./actix_web_api"]
