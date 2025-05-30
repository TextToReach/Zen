# Aşama 1: Derleme
FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev libc-dev
WORKDIR /app
COPY . .
RUN cargo build --release

# Aşama 2: Sadece binary ile küçük image
FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/release/ZenBackend .
ENTRYPOINT ["./ZenBackend"]