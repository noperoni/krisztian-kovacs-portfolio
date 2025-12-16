# ============================================
# Portfolio v2 - Leptos + Axum
# Multi-stage Dockerfile for minimal image size
# Target: < 50MB
# ============================================

# ============================================
# Stage 1: Builder
# ============================================
FROM rustlang/rust:nightly-alpine AS builder

# Install build dependencies
RUN apk update && \
    apk add --no-cache bash curl npm musl-dev binaryen

# Install sass for SCSS compilation
RUN npm install -g sass

# Install cargo-leptos
RUN curl --proto '=https' --tlsv1.3 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Add WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./
COPY build.rs ./

# Copy source and content
COPY src/ src/
COPY content/ content/
COPY style/ style/
COPY public/ public/
COPY migrations/ migrations/

# Build the application
RUN cargo leptos build --release -vv

# ============================================
# Stage 2: Runtime
# ============================================
FROM alpine:3.21 AS runtime

# Install minimal runtime dependencies (ca-certificates for TLS)
RUN apk add --no-cache ca-certificates

WORKDIR /app

# Copy the server binary
COPY --from=builder /app/target/release/portfolio /app/portfolio

# Copy the site assets (JS, WASM, CSS)
COPY --from=builder /app/target/site /app/site

# Copy Cargo.toml (needed by leptos at runtime for config)
COPY --from=builder /app/Cargo.toml /app/Cargo.toml

# Set environment variables
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"

# Expose the port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:3000/ || exit 1

# Run the server
CMD ["/app/portfolio"]
