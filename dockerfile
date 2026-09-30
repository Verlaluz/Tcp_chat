FROM rust:latest
WORKDIR /usr/src/tcp_chat
COPY . .
RUN cargo build --release

EXPOSE 8080

ENTRYPOINT ["./target/release/tcp_chat"]