FROM rust:1.60 as builder
RUN rustup default nightly
WORKDIR /usr/src/sf-motd
COPY src ./src
COPY Cargo.toml Cargo.lock ./

RUN cargo build

FROM rust:1.60-slim

COPY --from=builder /usr/src/sf-motd/target/debug/sf_motd /usr/local/bin/

ENV ROCKET_ADDRESS=0.0.0.0
ENV PORT=8000
EXPOSE 8000
COPY ./entrypoint.sh .
ENTRYPOINT ["./entrypoint.sh"]
