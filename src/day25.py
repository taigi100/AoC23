import networkx as nx
from math import prod

with open("day25.in", "r") as f:
    lines = [line.strip() for line in f.readlines()]

parsed = {}
for l in lines:
    parsed[l.split(":")[0]] = l.split(":")[1].strip().split()

G = nx.Graph()
for origin, dest in parsed.items():
    for d in dest:
        G.add_edge(origin, d)

cut = nx.minimum_edge_cut(G);
G.remove_edges_from(cut);

groups = list(nx.connected_components(G))
print(prod(len(i) for i in groups))