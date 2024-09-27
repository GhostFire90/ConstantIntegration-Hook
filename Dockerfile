FROM rust:latest AS build

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

ENV USER=ci-hooks
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /ci-hooks

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release


FROM scratch
COPY --from=build /etc/passwd /etc/passwd
COPY --from=build /etc/group /etc/group

COPY --from=build /ci-hooks/target/x86_64-unknown-linux-musl/release/constant-integration-hook ./
COPY ./.env .

USER ci-hooks:ci-hooks

VOLUME [ "/data" ]

CMD [ "./constant-integration-hook" ]