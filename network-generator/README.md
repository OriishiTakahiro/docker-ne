# Network Generator

This tool generates a YAML file which is specified a network.

This tool can generates some kinds network.

- random graph
- full mesh graph
- path grpah
- tree grpah

## Usage
```sh
$ docker build netgen -t .
# Generates path graph (5 nodes, docker image 'hoge')
$ docker run  -v $PWD:/root/artifacts netgen path -n 5 -i hoge
```

