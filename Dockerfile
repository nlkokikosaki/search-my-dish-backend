# 開発環境
FROM rust:1.63.0 AS builder

RUN apt update -y
RUN apt upgrade -y
RUN apt install -y libpq-dev

RUN cargo new backend
WORKDIR /backend

COPY /backend/Cargo.toml ./Cargo.toml
COPY /backend/Cargo.lock ./Cargo.lock

RUN cargo build --release

RUN rm src/*.rs

RUN cargo install diesel_cli sccache

COPY /backend/src ./src
COPY /backend/migrations ./migrations

# ENV DATABASE_URL=postgres://app:password@search-my-dish-db:5432/searchmydish
# ENV RUST_BACKTRACE=1

RUN cargo build --release

# RUN diesel migration run

EXPOSE 8080

CMD ["cargo", "run"]