FROM rust:1.75.0-slim as BUILD

WORKDIR /app-build/build

COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release

FROM ubuntu:latest

WORKDIR /app/rdsync

COPY --from=BUILD /app-build/build/target/release/rdsync .

COPY config.toml config.toml

#EXPOSE 7045

CMD [ "./rdsync" ]
