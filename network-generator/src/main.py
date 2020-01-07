import argparse
import networkx as nx
import matplotlib.pyplot as plt
from random import randint
from yaml import dump

# FORMAT
PREFIX_SPACE    = "2001:0db8:1b:ff{prefix}::/64"
ADDR_SPACE      = "2001:0db8:1b:ff{prefix}::ff{last_oct}/64"
ROUTER_ID_SPACE = "10.0.0.{node_id}"

# check paramters
parser = argparse.ArgumentParser(description='network parameters.')
parser.add_argument('--nodes', metavar='-n', type=int, help='Number of nodes.', default=64)
parser.add_argument('--proverbility', metavar='-p', type=float, help='Proverbility of make edges between 2 nodes.', default=0.08)
parser.add_argument('--yaml', metavar='-y', type=str, help='File name of YAML.', default='network.yml')
parser.add_argument('--figure', metavar='-f', type=str, help='File name of graph figure.', default='graph.png')
parser.add_argument('--image', metavar='-i', type=str, help='Docker image.', default='quagga-ca')
args = parser.parse_args()

# create graph
graph = nx.fast_gnp_random_graph(args.nodes, args.proverbility)

nodes = {}

# create nodes
for n in graph.nodes():
    name = "node" +  str(n)
    prefixes = [ PREFIX_SPACE.format(prefix = format(n, '02x')) ]
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

# set random node to CA
nodes[randint(0, len(nodes))]["is_ca"] = True

# create switches
for e in graph.edges(data=True):
    # start node
    nodes[e[0]]["interfaces"].append( {
        "name": "eth" + format(e[0], '03d') + format(e[1], '03d'),
        "prefix": ADDR_SPACE.format(prefix = format(e[0], '02x'), last_oct = format(e[1], '02x')),
        "area": "0.0.0.0",
        "switch": "s" + format(e[0], '03d') + format(e[1], '03d'),
        "cost": "1",
        "priority": "1",
    })
    # end node
    nodes[e[1]]["interfaces"].append( {
        "name": "eth" + format(e[1], '03d') + format(e[0], '03d'),
        "prefix": ADDR_SPACE.format(prefix = format(e[1], '02x'), last_oct = format(e[0], '02x')),
        "area": "0.0.0.0",
        "switch": "s" + format(e[0], '03d') + format(e[1], '03d'),
        "cost": "1",
        "priority": "1",
    })

nodes_list = []
for v in nodes.values():
    nodes_list.append(v)

# save YAML file
with open(args.yaml, "w") as f:
    f.write(dump(nodes_list))

# save network graph
nx.draw_networkx(graph)
plt.savefig(args.figure)
