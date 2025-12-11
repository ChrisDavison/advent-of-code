from utility import *
import networkx as nx

fname = "input/11"

G = nx.DiGraph()
con = defaultdict(list)
for line in open(fname).read().splitlines():
    frm, to = line.split(":")
    to = to.strip().split(" ")
    con[frm].extend(to)
    for t in to:
        G.add_edge(frm, t)
# print(list(nx.all_pairs_all_shortest_paths(G)))

c = 0
for p in nx.all_simple_paths(G, "you", "out"):
    print(f"{p = }")
    c += 1
print("part1", c)


# print(tracepath("you", "bbb"))
#
# print(tracepath("you", "eee"))
#
# ds = parse("11s")
# dd = parse("11s", show=False)

