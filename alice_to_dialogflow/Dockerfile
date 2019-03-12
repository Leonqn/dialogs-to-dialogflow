FROM rust:1.32.0

WORKDIR /app
COPY . .

RUN cargo install --path .

CMD ["alice_to_dialogflow"]