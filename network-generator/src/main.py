#!/usr/bin/env python3

import sys
import argparse
import networkx as nx
import matplotlib.pyplot as plt
from random import randint
from yaml import dump

# FORMAT
PREFIX_SPACE    = "2001:0db8:1b:ff{prefix}::/64"
ADDR_SPACE      = "2001:0db8:1b:ff{prefix}::ff{last_oct}/64"
ROUTER_ID_SPACE = "10.0.0.{node_id}"

BACKBONE_AREA   = "0.0.0.0"
ARTIFACT_PATH   = "/root/artifacts"

# handler functions for create a graph
def create_random_graph(args):
    return nx.fast_gnp_random_graph(args.nodes, args.proverbility)

def create_full_mesh_graph(args):
    return nx.fast_gnp_random_graph(args.nodes, 1)

def create_path_graph(args):
    return nx.path_graph(args.nodes)

def create_tree_graph(args):
    return nx.random_tree(args.nodes)

# check paramters
parser = argparse.ArgumentParser(description="Auto network topology generator.")
subparsers = parser.add_subparsers(dest="graph_type")
subparsers.required = True

# subcommand for random graph
parser_random = subparsers.add_parser("random", help="Create a random graph.")
parser_random.add_argument("-n", "--nodes", type=int, help="Number of nodes.", default=64)
parser_random.add_argument("-p", "--proverbility", type=float, help="Proverbility of make edges between 2 nodes.", default=0.08)
parser_random.add_argument("-i", "--image", type=str, help="Docker image.", default="quagga-ca")
parser_random.set_defaults(handler=create_random_graph)

# subcommand for full mesh graph
parser_full_mesh = subparsers.add_parser("fullmesh", help="Create a full mesh graph.")
parser_full_mesh.add_argument("-n", "--nodes", type=int, help="Number of nodes.", default=64)
parser_full_mesh.add_argument("-i", "--image", type=str, help="Docker image.", default="quagga-ca")
parser_full_mesh.set_defaults(handler=create_full_mesh_graph)

# subcommand for path graph
parser_path = subparsers.add_parser("path", help="Create a path graph.")
parser_path.add_argument("-n", "--nodes", type=int, help="Number of nodes.", default=64)
parser_path.add_argument("-i", "--image", type=str, help="Docker image.", default="quagga-ca")
parser_path.set_defaults(handler=create_path_graph)

# subcommand for tree graph
parser_tree = subparsers.add_parser("tree", help="Create a tree graph.")
parser_tree.add_argument("-n", "--nodes", type=int, help="Number of nodes.", default=64)
parser_tree.add_argument("-i", "--image", type=str, help="Docker image.", default="quagga-ca")
parser_tree.set_defaults(handler=create_tree_graph)

args = parser.parse_args()

# execute handler function
graph = args.handler(args)

nodes = {}
# create nodes
for n in graph.nodes():
    name = "node" +  str(n)
    prefixes = [ PREFIX_SPACE.format(prefix = format(n, "02x")) ]
    node_config = {
            "name": name,
            "image": args.image,
            "router_id": ROUTER_ID_SPACE.format(node_id = n),
            "instance_id": "0",
            "set_dummycert": False,
            "is_ca": False,
            "is_registered": True,
            "enabled_prefixes": prefixes,
            "interfaces": [],
        }
    nodes[n] = node_config

# set firs node as CA
nodes[0]["is_ca"] = True

# create switches
for e in graph.edges(data=True):
    # start node
    nodes[e[0]]["interfaces"].append( {
        "name": "eth" + format(e[0], "03d") + format(e[1], "03d"),
        "prefix": ADDR_SPACE.format(prefix = format(e[0], "02x"), last_oct = format(e[1], "02x")),
        "area": BACKBONE_AREA,
        "switch": "s" + format(e[0], "03d") + format(e[1], "03d"),
        "cost": "1",
        "priority": "1",
    })
    # end node
    nodes[e[1]]["interfaces"].append( {
        "name": "eth" + format(e[1], "03d") + format(e[0], "03d"),
        "prefix": ADDR_SPACE.format(prefix = format(e[1], "02x"), last_oct = format(e[0], "02x")),
        "area": BACKBONE_AREA,
        "switch": "s" + format(e[0], "03d") + format(e[1], "03d"),
        "cost": "1",
        "priority": "1",
    })

# generate yaml and svg file for each image
nodes_list = list(nodes.values())
for image in args.image.split(","):

    nodes_list = list(map(lambda n: {**n, "image": image}, nodes_list))

    # save YAML file
    with open( "{dir}/{type}-{image}-{nodes}.yaml".format(dir=ARTIFACT_PATH, type=args.graph_type, image=image, nodes=args.nodes)  , "w") as f:
        f.write(dump(nodes_list))

    # save network graph
    nx.draw_networkx(graph)
    plt.savefig( "{dir}/{type}-{image}-{nodes}.svg".format(dir=ARTIFACT_PATH, type=args.graph_type, image=image, nodes=args.nodes), format="svg" )
