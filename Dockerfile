# We use the latest Rust nightly release as base image
FROM rustlang/rust:nightly

ENV SQLX_OFFLINE true

RUN cargo install cargo-watch sqlx-cli  

WORKDIR /app

# ENTRYPOINT ["cargo"]
