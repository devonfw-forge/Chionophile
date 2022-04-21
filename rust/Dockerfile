FROM rust:latest

RUN mkdir /jtq
WORKDIR /jtq
COPY . .

RUN cargo install --path .

CMD ["jtqt-rust"]