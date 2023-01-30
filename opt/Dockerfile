FROM rust:1-slim-buster

ENV  LC_CTYPE=ja_JP.utf8 \
    LANG=ja_JP.utf8

RUN apt-get update \
    && apt-get install -y default-libmysqlclient-dev mariadb-client

WORKDIR /usr/src/myapp
