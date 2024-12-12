import utility as u
from collections import defaultdict
from pathlib import Path
from itertools import groupby
from argparse import ArgumentParser
import numpy as np


def dfs(graph, start, visited=None):
    if visited is None:
        visited = set()

    visited.add(start)
    # print(visited)
    for row_col in graph[start]:
        if row_col not in visited:
            dfs(graph, row_col, visited)
    return visited


def perimiter(graph, points):
    neighbours_seen = list()
    for p in points:
        for n in u.neighbors2(p):
            if n not in points:
                neighbours_seen.append(n)
    return neighbours_seen


def count_consecutive_groups(ll):
    n = 1
    ll = sorted(set(ll))
    # print(f"{ll = :}")
    prev, ll = ll[0], ll[1:]
    while ll:
        l, ll = ll[0], ll[1:]
        diff = l - prev
        # print(f"{prev = :} {l = :} {diff}")
        if diff != 1:
            n += 1
        prev = l
    return n


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("--debug", action="store_true", default=False)
    parser.add_argument("file", type=Path, nargs=1)
    args = parser.parse_args()

    DAYNUM = u.ints(Path(__file__).stem)[0]
    data = args.file[0].read_text().strip()

    # ------------------------------------------------------------
    #                        generate graph
    # ------------------------------------------------------------
    graph = defaultdict(list)
    # pad the grid with '.' so we don't need bounds check
    grid = [[ch for ch in "." + row + "."] for row in data.splitlines()]
    grid.insert(0, ["." for _ in range(len(grid[0]))])
    grid.insert(len(grid), ["." for _ in range(len(grid[0]))])
    grid = np.array(grid)
    for x, row in enumerate(grid):
        for y, ch in enumerate(row):
            if ch == ".":
                continue
            for x2, y2 in u.neighbors2((x, y)):
                if grid[x2][y2] == ch:
                    graph[(x, y)].append((x2, y2))
            if not graph[(x, y)]:
                graph[(x, y)] = []

    # ------------------------------------------------------------
    #                             solve
    # ------------------------------------------------------------
    done = set()
    areas = []
    for point in graph:
        if point not in done:
            ch = str(grid[point])
            done.add(point)
            out = dfs(graph, point)
            for p in out:
                done.add(p)

            areas.append((ch, out))

    tot = 0
    tot2 = 0
    for ch, area in areas:
        neighbours = perimiter(graph, area)
        perimiter_price = len(neighbours) * len(area)
        tot += perimiter_price

        verticals = []
        has_area_on_right = defaultdict(list)
        has_area_on_left = defaultdict(list)
        for y, ll in groupby(sorted(neighbours, key=lambda x: x[1]), lambda x: x[1]):
            for px, py in ll:
                point_right = (px, py + 1)
                point_left = (px, py - 1)
                if point_right in area:
                    has_area_on_right[py].append(px)
                if point_left in area:
                    has_area_on_left[py].append(px)

        sides = 0

        for k, xs in has_area_on_right.items():
            counts = count_consecutive_groups(xs)
            sides += counts

        for k, xs in has_area_on_left.items():
            sides += count_consecutive_groups(xs)

        side_price = 2 * sides * len(area)
        tot2 += side_price

    print(tot)
    print(tot2)
