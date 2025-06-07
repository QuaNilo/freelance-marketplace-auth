FROM rust:1.87 as builder
WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release || true
COPY src ./src
RUN cargo build --release

#Stage 2
FROM debian:bookworm
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/freelance-marketplace-auth /usr/local/bin/freelance-marketplace-auth

EXPOSE 45000

CMD ["freelance-marketplace-auth"]
