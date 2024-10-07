FROM rust:1.72 AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY . .

RUN cargo build --release

FROM debian::bullseye-slim

RUN apt-get update && apt-get install -y \
    libx11-dev \
    libxtst-dev \
    libxdo-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/keylog /usr/local/bin/keylog

CMD ["keylog"]
