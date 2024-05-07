FROM liuchong/rustup:nightly-musl AS builder
WORKDIR /rode-be-socket
COPY . .
RUN rustup install nightly
RUN cargo build --release

FROM debian:alpine AS final
WORKDIR /rode-be-socket
COPY --from=builder /rode-be-socket/target/release/rode-be-socket /rode-be-socket/rode-be-socket
RUN apt-get update \
    && apt-get install curl openjdk-8-jdk build-essential chromium-browser -y \
    && apt-get clean

CMD ["/rode-be-socket/rode-be-socket"]
