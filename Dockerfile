FROM rust:1.55

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/api"]