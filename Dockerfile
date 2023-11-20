FROM rust:latest

WORKDIR /app/db

VOLUME [ "/db" ]

COPY . .

RUN cargo build --release

CMD [ "./target/release/rust-database" ]
