FROM fluent/fluentd:v1.8-1

COPY ./fluent.conf /fluentd/etc/fluent.conf

RUN apk update
RUN apk add \
    python3

USER fluent
