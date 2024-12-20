from utility import PriorityQueue, ints, parse, Grid, only
from collections import namedtuple
from pathlib import Path


State = namedtuple("State", "pos, facing")


def bfs(graph, start, end):
    visited = set()
    q = PriorityQueue([[start]])
    # print(f"{graph=}")
    excludes = graph.findall("#")
    # print(f"{excludes=}")
    while q:
        path = q.pop()
        # print(f"{path=}")
        node = path[-1]
        # print(f"{node=} {graph[node]=} ")
        if node not in visited:
            assert graph[node] != "#", "Shouldn't be on a wall..."
            for neighbour in graph.neighbors(node, excluding=excludes):
                if graph[neighbour] == "#":
                    # print("skipping")
                    continue
                else:
                    new_path = list(path)
                    # print(f"{neighbour=} {graph[neighbour]=} ")
                    new_path.append(neighbour)
                    if neighbour == end:
                        return new_path
                    else:
                        q.add(new_path)
            visited.add(node)
    return None


# ------------------------------------------------------------
#                             Solve
# ------------------------------------------------------------
DAYNUM = ints(Path(__file__).stem)[0]
graph = Grid(parse("16s", show=0))
start = only(graph.findall("S"))
end = only(graph.findall("E"))
print(f"{start=}")
print(f"{end=}")
graph.print(highlight=[start, end], block="#")
print()
route = bfs(graph, start, end)
graph.print(highlight=route, block="#")
# bfs(graph, start, end, East)
