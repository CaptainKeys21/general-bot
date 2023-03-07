FROM rust:1.67

WORKDIR /usr/src/bot
COPY . .

RUN cargo install --path .
#RUN cargo run

CMD ["general-bot"]