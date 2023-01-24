FROM rust:1-buster


WORKDIR /app

COPY Cargo.toml .
COPY ./Cargo.lock .
COPY src ./src
RUN rustup default nightly
RUN cargo build --release

CMD ["./target/release/test-bench"]
