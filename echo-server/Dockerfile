FROM rust:1.63.0 as build

RUN apt-get update
RUN rustup component add rustfmt
WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:10.10-slim as server

RUN apt-get update
COPY --from=build /build/target/release/echo-server .
EXPOSE 3030

ENTRYPOINT ["./echo-server"]