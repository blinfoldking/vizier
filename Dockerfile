FROM alpine:3.20

COPY dist/vizier /usr/local/bin/vizier

ENV RUST_BACKTRACE=1

ENTRYPOINT ["vizier"]
