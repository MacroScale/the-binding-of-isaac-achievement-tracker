FROM rust:alpine as builder

ENV APP rust_server 

WORKDIR /usr/src/$APP
COPY ./app .

ENV RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=lld"

# Install build dependencies
RUN apk add --no-cache musl-dev
RUN apk add --no-cache openssl-dev
RUN apk add --no-cache clang
RUN apk add --no-cache lld
RUN apk add --no-cache build-base
RUN apk add --no-cache pkgconfig
RUN apk add --no-cache openssl-libs-static
RUN apk add --no-cache openssl-dev
RUN apk add --no-cache openssl

RUN cargo install --path .

FROM alpine:latest

# Install build dependencies
# RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*
# RUN apt-get update && apt install -y openssl 

ENV RUST_LOG=info

# Set the working directory
WORKDIR /usr/src/$APP

# Get output for release
COPY --from=builder /usr/local/cargo/bin/$APP /release/$APP

COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
COPY ./app/static /usr/local/bin/$APP/static 
COPY ./app/static ./static 

EXPOSE 3000 

# Installing nginx
RUN apk --no-cache add nginx

# Copy nginx config
COPY ./nginx/remote/site.conf /etc/nginx/conf.d/site.conf 
COPY ./nginx/remote/site.conf /etc/nginx/sites-enabled/default
COPY ./nginx/nginx.conf /etc/nginx

# Copy entrypoint script and give privilege to execute 
COPY entrypoint.sh /var/app/entrypoint.sh
RUN chmod -R 777 /var/app/entrypoint.sh

# Exposing nginx port
EXPOSE 80

# Invoke entrypoint.sh 
CMD ["/var/app/entrypoint.sh"]
