from collections import defaultdict
from itertools import product
import re
from utility import * 
import copy

SAMPLE = as_grid("""...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........""")
DATA = as_grid(open("input/21").read())

G = SAMPLE
xys = lambda: product(range(len(G[0])), range(len(G)))
S = first(Point2D(x,y) for (x,y) in xys() if G[x][y] == 'S')

def plot_grid(grid, points):
    g = copy.deepcopy(grid[:])
    for p in points:
        g[p.x][p.y] = 'O'
    for row in g:
        print(cat(row))


def part1(data=SAMPLE, n=6):
    G = data
    xys = lambda: product(range(len(G[0])), range(len(G)))
    S = first(Point2D(x,y) for (x,y) in xys() if G[x][y] == 'S')
    current = deque([S])
    for i in range(n):
        nc = deque()
        seen = set(current)
        while current: 
            point = current.popleft()
            for p in point.surrounding(4):
                if 0 <= p.x < len(G[0]) and 0 <= p.y < len(G):
                    if G[p.x][p.y] in '.S' and p not in seen:
                        nc.append(p)
                        seen.add(p)
        current = nc
    print(f"Part 1: {len(current)}")
    return len(current)

part1(DATA, n=64)
