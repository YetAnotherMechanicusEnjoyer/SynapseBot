FROM rust:slim-bookworm AS builder

WORKDIR /app

RUN apt-get update && \
  apt-get install -y \
  pkg-config \
  libssl-dev \
  build-essential && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock /app/

RUN mkdir src && echo "fn main() {}" > src/main.rs && \
  cargo build --release --locked && \
  rm -rf src

COPY . .

RUN cargo build --release --locked

FROM rust:slim-bookworm

RUN apt-get update && \
  apt-get install -y \
  libssl-dev && \
  apt-get clean

WORKDIR /app

COPY --from=builder /app/target/release/discord ./discord
COPY --from=builder /app/config.json ./config.json

RUN chmod +x ./discord

CMD [ "./discord" ]
