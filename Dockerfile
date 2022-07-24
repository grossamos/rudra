FROM rust:1-buster as builder

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install openssl -y

COPY ./Cargo.lock ./
COPY ./Cargo.toml ./
COPY ./src ./src

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM nginx:1.23.0-alpine

WORKDIR /app

# Unlink access log from stdout (to allow for analysis)
RUN rm /var/log/nginx/access.log
RUN echo rudra failed to connect to your specified service > /usr/share/nginx/html/50x.html

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rudra /
COPY ./nginx/nginx.conf /etc/nginx/nginx.conf

CMD ["/rudra"]
