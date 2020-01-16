FROM fluent/fluentd:v1.8-1

USER root
COPY ./fluent.conf /fluentd/etc/fluent.conf
RUN apk update
RUN apk add \
    python3
COPY ./log-summary.py /var/run/log-summary
ENV PATH "{$PATH}:/var/run"
