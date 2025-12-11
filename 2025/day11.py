from utility import *
from itertools import pairwise


class DFS:
    def __init__(self, edges):
        self.edges = edges
        self.cache: Dict[Tuple[str, str]|int, int] = {-1: 0}

    @staticmethod
    def from_file(filename):
        edges = {}
        for line in open(filename).read().splitlines():
            l, r = line.split(":")
            edges[l] = [p.strip() for p in r.split(" ")]
        return DFS(edges)

    def traverse(self, node: str, dest: str):
        if node == dest:
            return 1
        if (node, dest) in self.cache:
            self.cache[-1] += 1
            return self.cache[(node, dest)]
        ret = 0
        for node_ in self.edges.get(node, []):
            ret += self.traverse(node_, dest)
        self.cache[(node, dest)] = ret
        return ret

def p1(fname):
    col = chalk.red if not "s" in str(fname) else chalk.green
    print(col("part1"), DFS.from_file(fname).traverse("you", "out"))

def p2(fname):
    paths = [["svr", "fft", "dac", "out"], ["svr", "dac", "fft", "out"]]
    d = DFS.from_file(fname)
    tot = 0
    for path in paths:
        tot += product(d.traverse(*pair) for pair in pairwise(path))
    col = chalk.red if not "s" in str(fname) else chalk.green
    print(col("part2"), tot)


p1("input/11s")
p1("input/11")
p2("input/11s2")
p2("input/11")
