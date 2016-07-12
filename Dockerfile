FROM debian:jessie
MAINTAINER Danilo Bargen <mail@dbrgn.ch>

ENV RUST_VERSION=1.10.0

# Build base system
RUN apt-get update && \
  DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    libssl-dev && \
  curl -sO https://static.rust-lang.org/dist/rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz && \
  tar -xzf rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz && \
  ./rust-$RUST_VERSION-x86_64-unknown-linux-gnu/install.sh --without=rust-docs && \
  DEBIAN_FRONTEND=noninteractive apt-get remove --purge -y curl && \
  DEBIAN_FRONTEND=noninteractive apt-get autoremove -y && \
  rm -rf \
    rust-$RUST_VERSION-x86_64-unknown-linux-gnu \
    rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz \
    /var/lib/apt/lists/* \
    /tmp/* \
    /var/tmp/* && \
  mkdir /source

# Build bot
WORKDIR /source
COPY . /source
RUN cargo build --release && \
    cp target/release/telegram-groups-bot /usr/local/bin/telegram-groups-bot && \
    cd / && rm -rf /source

# Set runtime related environment variables
ENV RUST_LOG=warn,telegram_groups_bot=info \
    REDIS_HOST=redis

# Entry point
CMD ["/usr/local/bin/telegram-groups-bot"]
