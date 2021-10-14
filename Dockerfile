# Build rust
FROM rust:latest as build
MAINTAINER Eunjin Sul <eunjin.sul@gmail.com>

ENV PKG_CONFIG_ALLOW_CROSS=1
ARG APP

WORKDIR /usr/src/$APP
COPY . .

RUN cargo install sqlx-cli --no-default-features --features postgres
RUN cargo install --path .

# Make the release 
FROM gcr.io/distroless/cc

ARG APP

COPY --from=build /usr/local/cargo/bin/$APP usr/bin/app

ENTRYPOINT ["app"]
