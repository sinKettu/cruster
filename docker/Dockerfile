FROM ubuntu:22.04

RUN mkdir /opt/cruster \
    && adduser --shell /bin/bash --home /opt/cruster cruster \
    && chown -R cruster /opt/cruster \
    && chmod u+rwx /opt/cruster

RUN apt-get update \
    && apt-get install --no-install-recommends -y \
        build-essential=12.9ubuntu3 \
        git=1:2.34.1-1ubuntu1.6 \
        ca-certificates=20211016ubuntu0.22.04.1 \
        curl=7.81.0-1ubuntu1.7 \
        locales=2.35-0ubuntu3.1 \
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

ENTRYPOINT [ "cruster" ]
