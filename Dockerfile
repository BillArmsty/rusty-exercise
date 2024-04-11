FROM rust:alpine as builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*


COPY . .

RUN cargo build --release


FROM alpine:latest as runtime


COPY --from=builder /app/target/release/rusty_exercise /app/rusty_exercise

ENTRYPOINT ["/app/rusty_exercise"]