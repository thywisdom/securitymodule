FROM rust:1.85-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/ring-lwe /app/ring-lwe
EXPOSE 3000
CMD ["./ring-lwe"]
