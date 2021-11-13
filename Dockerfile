FROM nginx:latest

COPY nginx.conf /etc/nginx/nginx.conf

RUN apt-get update && apt-get install -y ssl-cert && make-ssl-cert generate-default-snakeoil

# Container setup
EXPOSE 4443/tcp
