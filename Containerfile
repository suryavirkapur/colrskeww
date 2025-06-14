FROM rust:1.87-slim as chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -r -u 1000 appuser

WORKDIR /app

COPY --from=builder /app/target/release/colrskeww /app/colrskeww

RUN chown -R appuser:appuser /app

USER appuser

EXPOSE 3000
# temp change
ENV RUST_LOG=debug
ENV HOST=0.0.0.0
ENV PORT=3000

CMD ["./colrskeww"]
