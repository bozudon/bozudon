FROM rust:1.70.0 AS builder

WORKDIR /

COPY ./src /src
COPY ./Cargo.toml /Cargo.toml
COPY ./Cargo.lock /Cargo.lock

RUN cargo build --release

FROM gcr.io/distroless/cc-debian11

COPY --from=builder /target/release/bozudon /bozudon

ENTRYPOINT ["/bozudon"]

EXPOSE 8080
