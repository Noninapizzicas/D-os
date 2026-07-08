# syntax=docker/dockerfile:1

# ============================================================================
# Etapa de compilación — `rust:1-bookworm` (Debian 12). IMPORTANTE: fijar la
# generación de Debian al mismo glibc que el runtime (`debian:12` /
# `distroless-cc-debian12`, glibc 2.36). `rust:1` a secas flota a Debian 13
# (glibc 2.39) y produce un binario que NO arranca en el runtime. El perfil
# `release` ya hace `strip`.
# ============================================================================
FROM rust:1-bookworm AS builder
WORKDIR /build

# No copiamos rust-toolchain.toml: el builder usa el toolchain de la imagen
# `rust:1` (evita un re-sync de rustup contra la red en cada build).
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
RUN cargo build --release --bin crawl4rs

# ============================================================================
# Imagen final — Debian slim + Chromium, para que el modo navegador
# (`crawl`, `deep`, `serve`) funcione de fábrica. Chromium pesa; si sólo
# necesitas la librería/`--html-file`/API sin navegador, usa el target
# `minimal` de más abajo (distroless, ~pocos MB).
# ============================================================================
FROM debian:12-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends chromium ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Usuario sin privilegios (Chromium headless corre con --no-sandbox).
RUN useradd --create-home --uid 10001 crawl4rs
COPY --from=builder /build/target/release/crawl4rs /usr/local/bin/crawl4rs

ENV CRAWL4RS_CHROME=/usr/bin/chromium \
    CRAWL4RS_JWT_SECRET=cambia-esto-en-produccion
USER crawl4rs
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/crawl4rs"]
CMD ["serve", "--host", "0.0.0.0", "--port", "8080"]

# ============================================================================
# Imagen mínima (distroless, sin navegador) — para uso como servicio de
# procesamiento de HTML ya obtenido o API sin `crawl <url>`.
#   docker build --target minimal -t crawl4rs:minimal .
# ============================================================================
FROM gcr.io/distroless/cc-debian12 AS minimal
COPY --from=builder /build/target/release/crawl4rs /usr/local/bin/crawl4rs
ENV CRAWL4RS_JWT_SECRET=cambia-esto-en-produccion
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/crawl4rs"]
CMD ["serve", "--host", "0.0.0.0", "--port", "8080"]
