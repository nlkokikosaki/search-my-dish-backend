# 開発環境
FROM rust:1.63.0 AS builder

RUN apt update -y
RUN apt upgrade -y
RUN apt install -y libpq-dev

RUN cargo new backend
WORKDIR /backend

# ENV CARGO_BUILD_TARGET_DIR=/tmp/target
# ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache

COPY /backend/Cargo.toml ./Cargo.toml
COPY /backend/Cargo.lock ./Cargo.lock

RUN cargo build --release

RUN rm src/*.rs

COPY /backend/src ./src
COPY /backend/migrations ./migrations

# ENV DATABASE_URL=postgres://root:password@db:5432/searchmydish

RUN cargo build --release

EXPOSE 8080

CMD ["cargo", "run"]