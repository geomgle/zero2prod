# We use the latest Rust nightly release as base image
# FROM debian:stable-slim
FROM rustlang/rust:nightly
MAINTAINER Eunjin Sul <eunjin.sul@gmail.com>

ENV SQLX_OFFLINE true
ENV HOME /home 

ARG UID
ARG GID
ARG DOCKER_USER=default_user

RUN apt-get update -qq && \
    apt-get install -y \
    inotify-tools

RUN mkdir -p /app

# Create a group and user
RUN groupadd -g $GID $DOCKER_USER && \
    useradd --no-create-home --shell /bin/bash --home $HOME -u $UID -g $GID $DOCKER_USER && \
    chown -R $DOCKER_USER:$DOCKER_USER /home && \
    chown -R $DOCKER_USER:$DOCKER_USER /app

USER $DOCKER_USER

RUN cargo install sqlx-cli

RUN mkdir -p /home/target

WORKDIR /app
