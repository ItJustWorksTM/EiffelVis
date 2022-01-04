FROM rust:alpine as backend-builder
MAINTAINER ItJustWorksTM ItJustWorksTM@aerostun.dev
ADD ./backend /opt/EiffelVis
WORKDIR /opt/EiffelVis
RUN apk add --no-cache musl-dev openssl openssl-dev
RUN cargo install --root ./dist/ --path .

FROM node:alpine as frontend-builder
MAINTAINER ItJustWorksTM ItJustWorksTM@aerostun.dev
ADD ./frontend /opt/EiffelVis
WORKDIR /opt/EiffelVis
RUN echo 'EIFFELVIS_URL = "@origin/api"' > .env \
 && echo 'EIFFELVIS_SSL = false' >> .env \
 && npm i \
 && npm run build


FROM nginx:alpine
MAINTAINER ItJustWorksTM ItJustWorksTM@aerostun.dev
RUN mv docker-entrypoint.sh docker-entrypoint-nginx.sh
ADD ./deployment/nginx/eiffelvis.conf /etc/nginx/conf.d/default.conf
ADD ./deployment/docker-entrypoint.sh /
COPY --from=backend-builder /opt/EiffelVis/dist/bin/* /usr/local/bin/
COPY --from=frontend-builder /opt/EiffelVis/public /var/www/html/EiffelVis

EXPOSE 80/tcp
ENTRYPOINT ["/docker-entrypoint.sh"]
CMD ["-r", "amqp://host.docker.internal:5672/%2f"]
