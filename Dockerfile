FROM rust:1.48.0-slim

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

CMD ["concrust-db"]
