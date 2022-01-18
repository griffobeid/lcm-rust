FROM rust:1.58-buster

WORKDIR /lcm

COPY . .

RUN cargo build --all
