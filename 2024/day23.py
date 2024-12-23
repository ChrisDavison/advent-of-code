from utility import *
from collections import defaultdict
import networkx as nx


def part1(filename):
    data = parse(filename, atoms, show=0)
    graph = defaultdict(list)
    for a, b in data:
        graph[a].append(b)
        graph[b].append(a)

    triples = set()
    for a, aa in graph.items():
        for b in aa:
            third = [c for c in graph[b] if c != a and c in aa]
            if third:
                for t in third:
                    triples.add(tuple(sorted((a, b, t))))

    with_t = [t for t in triples if any(x[0] == "t" for x in t)]
    print(f"part1 {filename}: {len(with_t)}")


def part2(filename):
    data = parse(filename, atoms, show=0)
    G = nx.Graph()
    G.add_edges_from(data)

    longest, _ = nx.max_weight_clique(G, weight=None)
    password = ",".join(sorted(longest))
    print(f"part2 {filename}: {password}")


if __name__ == "__main__":
    part1("23s")
    part1("23")
    part2("23s2")
    part2("23")
