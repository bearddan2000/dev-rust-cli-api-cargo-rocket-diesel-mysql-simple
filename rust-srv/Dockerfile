FROM rust:latest

WORKDIR /workspace

RUN apt update \
  && apt install -y libmariadb-dev libmariadb-dev-compat

WORKDIR /app

COPY bin .

RUN rustup override set nightly \
  && cargo install --path .

ENV WAIT_VERSION 2.7.2

ADD https://github.com/ufoscout/docker-compose-wait/releases/download/$WAIT_VERSION/wait /wait

RUN chmod +x /wait

CMD ["main"]
