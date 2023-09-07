FROM rust:1.70 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/general-bot /usr/local/bin/general-bot

RUN groupadd -r genbot && useradd -g genbot genbot
RUN chown genbot:genbot /usr/local/bin/general-bot

USER genbot

CMD [ "general-bot" ]
