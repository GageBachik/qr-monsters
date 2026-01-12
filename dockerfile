# Stage 1: Build the web app
FROM rust:1.83 AS builder

# Install wasm target
RUN rustup target add wasm32-unknown-unknown

# Install wasm-bindgen-cli (must match version in Cargo.lock)
RUN cargo install wasm-bindgen-cli --version 0.2.106

# Install dioxus-cli from source (binstall binaries require newer glibc)
RUN cargo install dioxus-cli --locked

WORKDIR /app

# Copy manifests first for better caching
COPY Cargo.toml Cargo.lock Dioxus.toml ./
COPY crates/core/Cargo.toml crates/core/Cargo.toml
COPY crates/qr/Cargo.toml crates/qr/Cargo.toml
COPY crates/avatar/Cargo.toml crates/avatar/Cargo.toml
COPY crates/app/Cargo.toml crates/app/Cargo.toml

# Create dummy source files for dependency caching
RUN mkdir -p crates/core/src crates/qr/src crates/avatar/src crates/app/src && \
    echo "fn main() {}" > crates/core/src/lib.rs && \
    echo "fn main() {}" > crates/qr/src/lib.rs && \
    echo "fn main() {}" > crates/avatar/src/lib.rs && \
    echo "fn main() {}" > crates/app/src/main.rs

# Build dependencies only (for caching)
RUN cargo build --package qrmonsters-app --features web --target wasm32-unknown-unknown --release || true

# Copy actual source code
COPY crates crates

# Touch source files to invalidate cache
RUN touch crates/core/src/lib.rs crates/qr/src/lib.rs crates/avatar/src/lib.rs crates/app/src/main.rs

# Build the web app
RUN dx build --package qrmonsters-app --platform web --release

# Stage 2: Serve with nginx
FROM nginx:alpine AS runtime

# Copy built static files
COPY --from=builder /app/target/dx/qrmonsters/release/web/public /usr/share/nginx/html

# Configure nginx for SPA routing
RUN echo 'server { \
    listen 80; \
    listen [::]:80; \
    root /usr/share/nginx/html; \
    index index.html; \
    location / { \
        try_files $uri $uri/ /index.html; \
    } \
    gzip on; \
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/wasm; \
}' > /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
