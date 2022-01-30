FROM rust:1.58 as builder
WORKDIR /usr/src/sly
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl1.1 libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/sly-bot /usr/local/bin/sly-bot
CMD ["sly-bot"]