from utility import parse, ints, Grid
from pathlib import Path
from collections import deque

DAYNUM = ints(Path(__file__).stem)[0]
data = parse("18s", ints)
print(data)
# data = Path(f"input/{DAYNUM}").read_text()


# ------------------------------------------------------------
#                             solve
# ------------------------------------------------------------
def bfs(graph, start, end):
    visited = set()
    q = deque([[start]])
    while q:
        path = q.popleft()
        # print(f"{path=}")
        node = path[-1]
        if node not in visited:
            for neighbour in graph.neighbors(node):
                if graph[neighbour] == "#":
                    continue
                new_path = list(path)
                new_path.append(neighbour)
                if neighbour == end:
                    return new_path
                else:
                    q.append(new_path)
            visited.add(node)
    return None


def simulate(coords, dim, n):
    grid = Grid.from_dimensions(dim[0] + 1, dim[1] + 1)
    grid.print()
    for coord in coords[:n]:
        grid[coord] = "#"
    print()
    grid.print()
    route = bfs(grid, (0, 0), dim)
    # print(f"{route=}")
    print(len(set(route) - {(0, 0)}))


def simulate_part2(coords, dim):
    grid = Grid.from_dimensions(dim[0] + 1, dim[1] + 1)
    grid.print()
    route = []
    for coord in coords:
        grid[coord] = "#"
        if not route or coord in route:
            route = bfs(grid, (0, 0), dim)
        if not route:
            print("NO ROUTE")
            print(coord)
            break
    print()
    grid.print()


sample = parse("18s", ints, show=0)
data = parse("18", ints, show=0)

print()
simulate(sample, (6, 6), 12)

print()
simulate(data, (70, 70), 1024)

print()
simulate_part2(sample, (6, 6))

print()
simulate_part2(data, (70, 70))
