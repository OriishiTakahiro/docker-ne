#!/bin/sh
GRAPH_TYPES=(
  "random"
  "fullmesh"
  "path"
)
NODE_N_LIST=(
  "8"
  "16"
  "32"
  "48"
  "64"
  "80"
  "96"
  "128"
  "144"
  "160"
)
IMAGES="quagga-original,quaggga-pki"
YAML_DIR="${PWD}/networks"

for TYPE in ${GRAPH_TYPES[@]}; do
  for NODE_N in ${NODE_N_LIST[@]}; do
    echo "${TYPE}: ${NODE_N}"
    docker run  -v ${YAML_DIR}:/root/artifacts netgen $TYPE -n ${NODE_N} -i ${IMAGES}
  done
done
