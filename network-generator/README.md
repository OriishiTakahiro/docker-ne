# Network Generator

This tool generates a YAML file which is specified a network.

This tool can generates some kinds network.

- random graph
- full mesh graph
- path grpah
- tree grpah

## Usage

Build docker image.

```sh
$ docker build netgen -t .
```

Generates a path graph. (five nodes, hoge images)

```sh
$ docker run  -v $PWD:/root/artifacts netgen path -n 5 -i hoge
```

You can generate graph with some images at same time.

```sh
$ docker run  -v $PWD:/root/artifacts netgen random -n 5 -i hoge,foo,bar
```
