FROM rust:bookworm AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends libpq-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
RUN cargo build --release --locked

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libpq5 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --create-home app
WORKDIR /app
COPY --from=builder /app/target/release/cheomseongd /usr/local/bin/cheomseongd
USER app
ENTRYPOINT ["cheomseongd"]
CMD ["start"]
