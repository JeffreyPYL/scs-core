FROM rust:1.66

WORKDIR /app
COPY src /app/src
COPY Cargo.lock /app
COPY Cargo.toml /app

RUN cargo build --release

CMD ["cargo", "run", "--release"]
