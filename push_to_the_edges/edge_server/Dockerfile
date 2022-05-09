FROM rust:latest

RUN mkdir /edge_server
WORKDIR /edge_server
COPY . .

RUN cargo install --path cache_manager 
RUN cargo install --path server 

CMD ["server"]
