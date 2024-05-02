FROM mcr.microsoft.com/devcontainers/rust

RUN apt-get update \
  && apt-get install -y -q \
  git-flow \
  ca-certificates \
  locales \
  libpq-dev \
  gnupg \
  apt-transport-https\
  libssl-dev \
  build-essential \
  && echo "ja_JP UTF-8" > /etc/locale.gen \
  && locale-gen \
  && echo "install rust tools" \
  && cargo install cargo-watch cargo-make diesel_cli \
  && cargo install diesel_cli --no-default-features --features postgres


