FROM rust:latest AS build

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR /ci-hooks

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release


FROM python:3.12-alpine

VOLUME [ "/data" ]
VOLUME [ "/.git-credentials" ]


RUN apk update
RUN apk add git bash
RUN pip install poetry

COPY ./.env .
COPY ./event-script .
COPY ./startup-script .
COPY --from=build /ci-hooks/target/x86_64-unknown-linux-musl/release/constant-integration-hook ./




CMD [ "./startup-script" ]