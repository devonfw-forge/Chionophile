FROM rust:latest

RUN mkdir /edge_server
WORKDIR /edge_server
COPY . .

RUN cargo build --release 
RUN cargo run --release --bin server