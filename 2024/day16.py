import utility as u
import string
import itertools as it
from collections import defaultdict
from pathlib import Path
from simple_chalk import chalk

DAYNUM = u.ints(Path(__file__).stem)[0]
data = Path(f"input/{DAYNUM}s").read_text()
data = Path(f"input/{DAYNUM}s2").read_text()
# data = Path(f"input/{DAYNUM}").read_text()

grid = dict()
start = end = None
maxx = maxy = 0
for (x, y), ch in u.enumerated_grid(data.splitlines()):
    grid[(x, y)] = ch
    maxx = max(x, maxx)
    maxy = max(y, maxy)
    if ch == "S":
        start = (x, y)
    if ch == "E":
        end = (x, y)

print(f"{maxx=} {maxy=} ")
graph = defaultdict(list)
for (x, y), ch in grid.items():
    if ch == "#":
        continue
    for x2, y2 in u.neighbors2((x, y)):
        if grid[(x2, y2)] != "#":
            graph[(x, y)].append((x2, y2))


def dfs(graph, start, end, direction, cost, visited=None):
    if visited is None:
        visited = dict()

    if start == end:
        visited[end] = True
        return cost, visited.keys(), True

    visited[start] = True

    min_score = None
    # print(f"{direction}")
    movecost = 0
    for neighbour in graph[start]:
        delta = u.Point2D(*neighbour) - u.Point2D(*start)
        # print(f"{delta} {direction}")
        if delta != u.Point2D(*direction):
            movecost = 1000 + 1
        else:
            movecost = 1
        # print(start, neighbour, delta)
        if neighbour not in visited:
            score, visited_, final = dfs(
                graph, neighbour, end, direction, cost + movecost, visited
            )
            if final:
                return score, visited.keys(), True
            if not score:
                continue
            if min_score is not None:
                min_score = min(score, min_score)
            else:
                min_score = score
            print(score, min_score)
    # if we don't remove start, we also include dead ends
    del visited[start]

    return min_score, visited, False


score, path, final = dfs(graph, start, end, u.East, 0)
print(f"{start=} {end=} {score=} {final=} ")
print(f"{path=}")
output = [["." for _ in range(maxx + 1)] for _ in range(maxy + 1)]
for (x, y), ch in grid.items():
    output[y][x] = ch
letters = list(string.digits + string.ascii_lowercase + string.ascii_uppercase)
for i, v in enumerate(path):
    x, y = v
    output[y][x] = chalk.red(letters[i % len(letters)])
print("\n".join("".join(str(s) for s in row) for row in output))
