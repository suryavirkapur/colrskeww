FROM rust:1.88-slim AS builder
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse \
    CARGO_NET_RETRY=10 \
    CARGO_HTTP_TIMEOUT=600 \
    CARGO_HTTP_MULTIPLEXING=false
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo build --release --bin colrskeww

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --create-home --uid 1000 appuser

WORKDIR /app

COPY --from=builder /app/target/release/colrskeww /app/colrskeww

RUN chown -R appuser:appuser /app

USER appuser

EXPOSE 3000
ENV RUST_LOG=info
ENV HOST=0.0.0.0
ENV PORT=3000

CMD ["./colrskeww"]
