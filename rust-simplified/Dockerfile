FROM ekidd/rust-musl-builder:latest as build
WORKDIR /build

ADD --chown=rust:rust . ./

RUN cargo build --release

FROM alpine:latest
RUN apk --no-cache add ca-certificates

WORKDIR /app

COPY --from=build /build/target/x86_64-unknown-linux-musl/release .

CMD [ "/app/jtq-rust" ]