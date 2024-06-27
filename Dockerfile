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
    && apt-get install curl openjdk-8-jdk build-essential python3 -y \
    && apt-get clean
    RUN curl -LO https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb
    RUN apt-get install -y ./google-chrome-stable_current_amd64.deb
    RUN rm google-chrome-stable_current_amd64.deb
ENTRYPOINT ["/rode-be-socket/rode-be-socket"]