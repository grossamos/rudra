FROM ubuntu:latest as builder

RUN apt update && apt install hugo -y
WORKDIR /tmp
COPY ./ /tmp
RUN hugo --minify

FROM nginx

EXPOSE 80
COPY --from=builder /tmp/public /usr/share/nginx/html
