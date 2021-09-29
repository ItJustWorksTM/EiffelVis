
FROM rust:alpine as builder
MAINTAINER ItJustWorksTM ItJustWorksTM@aerostun.dev
ADD . /opt/EiffelVis
WORKDIR /opt/EiffelVis
RUN apk add --no-cache musl-dev openssl openssl-dev
RUN cargo install --root ./dist/ --path .

FROM alpine
MAINTAINER ItJustWorksTM ItJustWorksTM@aerostun.dev
COPY --from=builder /opt/EiffelVis/dist/bin/* /usr/local/bin/
ENTRYPOINT /usr/local/bin/eiffelvis
