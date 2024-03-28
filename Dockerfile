FROM rust:alpine

RUN adduser -D robbit
RUN apk add --no-cache musl-dev

WORKDIR /home/robbit/
COPY ./ ./
RUN chown robbit ./*

USER robbit

RUN cargo build -r

CMD ./target/release/robbit
