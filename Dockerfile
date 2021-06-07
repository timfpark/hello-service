FROM rust:1.52 as build

COPY ./ ./

RUN cargo build --release

RUN mkdir -p /build
RUN cp target/release/hello-service /build/

FROM ubuntu:18.04

COPY --from=build /build/hello-service /

CMD ["/hello-service"]
