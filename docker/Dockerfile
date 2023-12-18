FROM ubuntu:22.04 as build

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

WORKDIR /root

COPY src cruster-src/src
COPY Cargo.toml cruster-src/
COPY Cargo.lock cruster-src/

ENV PATH="/root/.cargo/bin:$PATH"

RUN curl --proto '=https' --tlsv1.2 -sSf -o rustup.sh https://sh.rustup.rs \
    && sh rustup.sh -y

RUN cd cruster-src \
    && cargo build --release \
    && cp target/release/cruster /cruster



FROM ubuntu:22.04

RUN mkdir /opt/cruster \
    && useradd -u 4242 --shell /bin/bash --home /opt/cruster cruster \
    && chmod u+rwx /opt/cruster

COPY --from=build /cruster /opt/cruster/cruster

RUN chown -R cruster /opt/cruster \
    && chmod +x /opt/cruster/cruster

USER cruster

WORKDIR /opt/cruster

ENTRYPOINT [ "/opt/cruster/cruster" ]

CMD [ "help" ]
