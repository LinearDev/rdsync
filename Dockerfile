FROM rust:latest

WORKDIR /app/db

VOLUME [ "/db" ]

COPY ./db ./db
COPY config.toml config.toml

RUN cargo build --release

CMD [ "./target/release/rust-database" ]
