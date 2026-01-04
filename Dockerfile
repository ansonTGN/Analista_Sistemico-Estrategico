# ETAPA 1: Builder
FROM rust:1.75-slim-bookworm as builder

# Instalar dependencias de compilación para C++ (pdf-extract necesita esto)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libpoppler-glib-dev \
    libglib2.0-dev \
    libc6-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./

# Truco para cachear dependencias: Compilar un main vacío
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copiar el código real y compilar
COPY src ./src
COPY templates ./templates
RUN touch src/main.rs
RUN cargo build --release

# ETAPA 2: Runtime (Imagen ligera final)
FROM debian:bookworm-slim

# Instalar dependencias de ejecución
RUN apt-get update && apt-get install -y \
    libpoppler-glib8 \
    libglib2.0-0 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar binario y templates
COPY --from=builder /usr/src/app/target/release/analista_sistemico .
COPY --from=builder /usr/src/app/templates ./templates

# Crear carpeta tmp
RUN mkdir -p /tmp

# Arrancar
CMD ["./analista_sistemico"]