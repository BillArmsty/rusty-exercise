FROM rust as builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*


COPY . .

RUN cargo build --release


FROM rust as runtime

WORKDIR /app

COPY --from=builder /app/target/release/rusty_exercise .

ENTRYPOINT ["/app/rusty_exercise"]