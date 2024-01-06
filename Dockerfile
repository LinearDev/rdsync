FROM rust:1.75.0-slim as BUILD

WORKDIR /app-build/build

COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release

FROM alpine:latest

WORKDIR /app/rdsync
VOLUME [ "/db" ]

COPY --from=BUILD /app-build/build/target/release/rdsync /app/rdsync/rdsync

COPY config.toml config.toml

CMD [ "./rdsync" ]
