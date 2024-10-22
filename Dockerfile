FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin amcoff-bostader-api

FROM debian:bookworm AS pdfium
WORKDIR /app
RUN apt-get update && apt-get install -y curl
RUN curl https://github.com/bblanchon/pdfium-binaries/releases/download/chromium%2F6569/pdfium-linux-x64.tgz -L -o pdfium-linux-x64.tgz
RUN tar -xzf pdfium-linux-x64.tgz

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev
COPY --from=pdfium /app/lib/libpdfium.so /app/libpdfium.so
COPY --from=builder /app/target/release/amcoff-bostader-api /usr/local/bin
EXPOSE 8000
CMD ["/usr/local/bin/amcoff-bostader-api"]
