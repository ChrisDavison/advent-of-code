#!/usr/bin/env python3
from prelude import *
from collections import defaultdict, Counter
from intcode import IntCode
import pyperclip as clip
import logging
import networkx as nx


def orbits_of(node, graph):
    return len(graph[node].keys()) + sum([orbits_of(k, graph) for k in graph[node]])


def make_graph(lines):
    g = nx.DiGraph()
    for line in lines:
        start, end = line.split(')')[:2]
        g.add_node(start)
        g.add_node(end)
        g.add_edge(start, end)
    return g


if __name__ == "__main__":
    # part1
    g = make_graph(open('input/06').read().splitlines())
    tot = 0
    for node in g.nodes():
        tot += orbits_of(node, g)
    print(f"2019 6.1 -> {tot}")
    clip.copy(tot)

    # part2
    ancest = nx.lowest_common_ancestor(g, 'YOU', 'SAN')
    me_to_ancest = next(nx.shortest_simple_paths(g, ancest, 'YOU'))[1:-1]
    san_to_ancest = next(nx.shortest_simple_paths(g, ancest, 'SAN'))[1:-1]
    n_transfers = len(me_to_ancest) + len(san_to_ancest)

    print(f"2019 6.2 -> {n_transfers}")
    clip.copy(n_transfers)

