FROM rust:1 as builder

WORKDIR /app

COPY . .

RUN cargo install --path .

FROM debian:bullseye as runner

RUN apt update
RUN apt install -y libpq-dev

COPY --from=builder /usr/local/cargo/bin/rusty-todo /usr/local/bin/rusty-todo

ENV ROCKET_ADDRESS=0.0.0.0

EXPOSE 8000

CMD ["rusty-todo"]
