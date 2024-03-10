# Build stage
FROM rust:1.76-bookworm AS builder
WORKDIR /usr/src/sample-web-rust
COPY Cargo.toml Cargo.lock .
COPY src src
RUN cargo install --path .


# Deploy stage
FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/sample-web-rust /usr/local/bin/sample-web-rust
ENV ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=8000
EXPOSE 8000
CMD ["sample-web-rust"]
