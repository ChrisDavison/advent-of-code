from utility import *
import networkx as nx

def read_graph(filename):
    G = nx.DiGraph()
    con = defaultdict(list)
    for line in open(filename).read().splitlines():
        frm, to = line.split(":")
        to = to.strip().split(" ")
        con[frm].extend(to)
        for t in to:
            G.add_edge(frm, t)
    return G

c = 0
for p in nx.all_simple_paths(read_graph("input/11"), "you", "out"):
    c += 1
print("part1", c)

# Part 2 is too slow to just use basic NetworkX
c = 0
for p in nx.all_simple_paths(read_graph("input/11s2"), "svr", "out"):
    if "fft" in p and "dac" in p:
        c += 1
print("part2", c)

