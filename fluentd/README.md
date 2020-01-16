# Docker Log Manager by Fluentd

## Build Image

```sh
$ docker build -t td-agent .
```

## Run Container

```sh
$ docker run -d -p 24224:24224 -p 24224:24224/udp -v $PWD/test-log:/tmp/log/fluentd --name td-agent td-agent
```

## Show Log Summary

```sh
# Human readable format
$ docker exec td-agent log-summary
# CSV format
$ docker exec td-agent log-summary -f csv
```
