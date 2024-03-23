FROM alpine:latest

# Install build dependencies
# RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*
# RUN apt-get update && apt install -y openssl 

ENV RUST_LOG=info

# Set the working directory
WORKDIR /usr/src/$APP

COPY ./release /usr/local/bin/$APP
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
