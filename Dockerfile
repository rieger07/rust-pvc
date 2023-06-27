FROM rust:alpine3.18 as build
RUN mkdir -p /build
RUN apk --no-cache add openssl build-base
WORKDIR /build
COPY . /build/
RUN cargo build --release

FROM alpine:3.18.2
WORKDIR /root/
COPY --from=0 /build/target/release/rust-pvc ./rust-pvc
CMD [ "./rust-pvc" ]