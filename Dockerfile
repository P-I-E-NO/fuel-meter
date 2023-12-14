FROM rust:1.74-buster as builder

WORKDIR /code
COPY ./src /code/src
COPY ./Cargo.toml /code/Cargo.toml
RUN cargo build --bins --release

FROM debian:12-slim
COPY --from=builder /code/target/release/fuel-meter /
ENTRYPOINT [ "./fuel-meter" ]