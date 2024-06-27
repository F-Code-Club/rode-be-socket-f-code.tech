# FROM liuchong/rustup:nightly-musl AS builder
FROM rustlang/rust:nightly-bullseye-slim AS builder
WORKDIR /rode-be-build
COPY . .
# RUN rustup install nightly
ARG DB_URL
ENV DATABASE_URL=${DB_URL}
RUN cargo build --release

FROM ubuntu:22.04 AS final
WORKDIR /rode-be-socket
COPY --from=builder /rode-be-build/target/release/rode-be-socket /rode-be-socket
RUN apt-get update \
    && apt-get install curl openjdk-8-jdk build-essential python3 libssl-dev -y \
    && apt-get clean
    
ENTRYPOINT ["/rode-be-socket/rode-be-socket"]
