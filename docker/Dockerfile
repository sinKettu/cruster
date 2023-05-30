FROM ubuntu:22.04

RUN mkdir /opt/cruster \
    && useradd -u 4242 --shell /bin/bash --home /opt/cruster cruster \
    && chown -R cruster /opt/cruster \
    && chmod u+rwx /opt/cruster

RUN apt-get update \
    && apt-get install --no-install-recommends -y \
        build-essential \
        git \
        ca-certificates \
        curl \
        locales \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && locale-gen en_US.UTF-8

ENV LANG=en_US.UTF-8
ENV LANGUAGE=en_US.UTF-8

WORKDIR /opt/cruster

USER cruster

ENV PATH="/opt/cruster/.cargo/bin:$PATH"

RUN curl --proto '=https' --tlsv1.2 -sSf -o rustup.sh https://sh.rustup.rs \
    && sh rustup.sh -y \
    && mkdir cruster-src

COPY src cruster-src/src
COPY Cargo.toml cruster-src/
COPY Cargo.lock cruster-src/

RUN cargo install \
    --path ./cruster-src

USER root

RUN rm -rf /opt/cruster/.rustup \
    && rm -rf /opt/cruster/.cargo/registry \
    && rm -rf /opt/cruster/cruster-src

USER cruster

ENTRYPOINT [ "cruster" ]
