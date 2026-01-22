FROM rust:1.92

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
ENV RUST_LOG=info

WORKDIR /app
COPY . .

EXPOSE 8080
RUN cargo build --release

CMD ["/app/target/release/listener"]