FROM rust:bookworm as builder

WORKDIR /code
COPY ./src /code/src
COPY ./Cargo.toml /code/Cargo.toml
RUN cargo build --bins --release

FROM debian:bookworm

RUN apt update
RUN apt install -y ca-certificates tzdata openssl libssl3 wget && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /app
RUN groupadd piffo && useradd -g piffo piffo 
RUN chown -R piffo:piffo /app
COPY --from=builder /code/target/release/fuel-meter /app/fuel-meter

WORKDIR /app
USER piffo

ENTRYPOINT [ "./fuel-meter" ]