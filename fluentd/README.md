# Docker Log Manager by Fluentd

## Build Image

```sh
$ docker build -t td-agent .
```

## Run Container

```sh
$ docker run --rm -d -p 24224:24224 -p 24224:24224/udp -v $PWD/test-log:/tmp/log/fluentd --name td-agent td-agent
```

## Show Log Summary

```sh
# Human readable format
$ docker exec td-agent log-summary
# CSV format
$ docker exec td-agent log-summary -f csv
```

## Sho td-agent Log

td-agent aggregates log of all routers.
You can show router's log with 'show-log.rb' script.

```sh
# You can see node3's log in 'path-quagga-pki-5' network.
$ docker logs td-agent | grep docker.path-quagga-pki-5.node3 | ./show-log.rb
```
