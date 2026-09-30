# syntax=docker/dockerfile:1

FROM rust:1-trixie AS builder

RUN rustup target add wasm32-unknown-unknown

# wasm-bindgen-cli must match the wasm-bindgen version pinned in Cargo.toml
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
    && cargo binstall -y --locked cargo-leptos@0.3.8 wasm-bindgen-cli@0.2.127

WORKDIR /app
COPY . .
RUN cargo leptos build --release

FROM debian:trixie-slim

WORKDIR /app
COPY --from=builder /app/target/release/jpn /app/jpn
COPY --from=builder /app/target/site /app/site-root

ENV LEPTOS_OUTPUT_NAME=jpn \
    LEPTOS_SITE_ROOT=site-root \
    LEPTOS_SITE_PKG_DIR=pkg \
    LEPTOS_SITE_ADDR=0.0.0.0:8080 \
    LEPTOS_ENV=PROD \
    RUST_LOG=info

EXPOSE 8080
CMD ["/app/jpn"]
