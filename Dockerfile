FROM rust:1.74-alpine3.17 as builder

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconf git libpq-dev

ENV SYSROOT=/dummy
ENV LIBPQ_STATIC=1

WORKDIR /code
COPY ./src /code/src
COPY ./Cargo.toml /code/Cargo.toml
RUN cargo build --bins --release

FROM scratch
COPY --from=builder /code/target/release/fuel-meter /
ENTRYPOINT [ "./fuel-meter" ]