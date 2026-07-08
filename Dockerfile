# syntax=docker/dockerfile:1

# ---- Etapa de compilación ----
FROM rust:1-slim AS builder
WORKDIR /build

# Cachea dependencias: copia manifiestos y compila un esqueleto.
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY crates ./crates
RUN cargo build --release --bin crawl4rs \
    && strip target/release/crawl4rs

# ---- Etapa de compilación de imagen ligera ----
# distroless/cc trae glibc (el binario enlaza dinámicamente). El binario en sí
# ronda los ~6 MB; la imagen final queda muy por debajo de imágenes basadas en
# Debian/Ubuntu completas.
#
# NOTA: el modo navegador (`crawl <url>`, `deep`, `serve`) necesita un
# Chromium/Chrome accesible en tiempo de ejecución. Esta imagen NO lo incluye
# para mantenerse mínima; proporciónalo montando el binario y apuntando
# `CRAWL4RS_CHROME` a él, o usa una imagen base que ya contenga Chromium.
FROM gcr.io/distroless/cc-debian12 AS runtime
COPY --from=builder /build/target/release/crawl4rs /usr/local/bin/crawl4rs

ENV CRAWL4RS_JWT_SECRET=cambia-esto-en-produccion
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/crawl4rs"]
CMD ["serve", "--host", "0.0.0.0", "--port", "8080"]
