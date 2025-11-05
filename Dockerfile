FROM rust:1.91-slim as wizrd-builder
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim as wizrd
COPY --from=wizrd-builder /target/release/wizrd-backend .
COPY Rocket.toml .
EXPOSE 8000
CMD ["/wizrd-backend"]
