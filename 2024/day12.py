import utility as u
from collections import defaultdict
from pathlib import Path
from argparse import ArgumentParser
import numpy as np


def dfs(graph, start, ch, visited=None):
    if visited is None:
        visited = set()

    if graph[start] != ch:
        return 0

    visited.add(start)
    pathcount = 0
    for xy in u.neighbors2(start):
        if xy not in visited:
            pathcount += dfs(graph, xy, ch, visited)
    visited.remove(start)
    return pathcount


if __name__ == "__main__":
    parser = ArgumentParser()
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("-s", "--sample", action="store_true")
    mode.add_argument("-d", "--data", action="store_true")
    parser.add_argument("-n", default=25, type=int)
    parser.add_argument("--debug", action="store_true", default=False)
    parser.add_argument("input", nargs="*")
    args = parser.parse_args()

    DAYNUM = u.ints(Path(__file__).stem)[0]
    if args.sample:
        data = Path(f"input/{DAYNUM:02d}s").read_text().strip()
    elif args.data:
        data = Path(f"input/{DAYNUM:02d}").read_text().strip()
    elif args.input:
        data = " ".join(args.input)
    else:
        data = input("input: ")

    # ------------------------------------------------------------
    #                        generate graph
    # ------------------------------------------------------------
    graph = defaultdict(list)
    grid = np.array([[ch for ch in row] for row in data.splitlines()])
    for x, row in enumerate(grid):
        for y, ch in enumerate(row):
            for x2, y2 in u.neighbors2((x, y)):
                if x2 >= 0 and x2 < grid.shape[0] and y2 >= 0 and y2 < grid.shape[1]:
                    if grid[x2][y2] == ch:
                        graph[(x, y)].append((x2, y2))

    # ------------------------------------------------------------
    #                             solve
    # ------------------------------------------------------------
    print(data)
    print(graph)
