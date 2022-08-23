# 開発環境
FROM rust:latest
WORKDIR /backend
RUN cargo install cargo-watch
RUN apt install -y libpq-dev
RUN cargo install diesel_cli