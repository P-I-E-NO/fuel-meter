FROM rust:1.73
RUN useradd --user-group --system --create-home --no-log-init dockerissimo
WORKDIR /code
RUN chown dockerissimo:dockerissimo -R /code
USER dockerissimo
COPY --chown=dockerissimo:dockerissimo ./src /code/src
COPY --chown=dockerissimo:dockerissimo ./Cargo.toml /code/Cargo.toml
RUN cargo install cargo-watch
RUN cargo build
# ENTRYPOINT ["./wait-for", "db:", "--", "nodemon"] # for dev purposes only