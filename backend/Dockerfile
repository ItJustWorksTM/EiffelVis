
FROM rust:alpine as builder
MAINTAINER ItJustWorksTM ItJustWorksTM@aerostun.dev
ADD . /opt/EiffelVis
WORKDIR /opt/EiffelVis
RUN cargo install --root ./dist/ --path .

FROM alpine
MAINTAINER ItJustWorksTM ItJustWorksTM@aerostun.dev
COPY --from=builder /opt/EiffelVis/dist/bin/* /usr/local/bin/
