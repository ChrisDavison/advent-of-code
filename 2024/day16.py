from utility import *
from heapq import heappop, heappush


def bfs(graph, start, end):
    excludes = graph.findall("#")
    heap = [(0, 0, *start)]  # score, direction, position
    visited = set()
    while heap:
        score, dir, i, j = heappop(heap)
        # if we've reached the end of the maze, we don't need to search more
        if (i, j) == end:
            break

        # if we come into a point from the same direction as we previously have,
        # we don't need to recalculate
        if (dir, i, j) in visited:
            continue

        visited.add((dir, i, j))

        dx, dy = directions4[dir]

        x, y = i + dx, j + dy
        if (x, y) not in excludes and (dir, x, y) not in visited:
            #  continuing 'forward' (in whatever we were going)
            # so att that direction to the heap with a score +1
            heappush(heap, (score + 1, dir, x, y))

        left = (dir - 1) % 4
        if (left, i, j) not in visited:
            # add a turn candidate to the heap ukeeping the same position,
            # but with a turn)
            heappush(heap, (score + 1000, left, i, j))

        right = (dir + 1) % 4
        if (right, i, j) not in visited:
            # ...and the other turn candidate
            heappush(heap, (score + 1000, right, i, j))
    return score


def bfs_tracking_path(graph, start, end):
    excludes = graph.findall("#")
    heap = [(0, 0, *start, {start})]  # score, direction, position, path_points
    visited = {}
    lowest_score = None
    winning_path_list = []  # keep track of all the best paths
    winning_paths = set()  # keep track of seats that are on any best path

    def visitable(d, i, j, score):
        # check if the next current point is a decent point to check
        prev_score = visited.get((dir, i, j))
        if prev_score and prev_score < score:
            # we don't want to continue investigating this occurence of a point
            # as we've previously found a better path
            return False
        visited[(dir, i, j)] = score
        return True

    while heap:
        score, dir, i, j, path = heappop(heap)
        if lowest_score and lowest_score < score:
            # we've found a route that is more expensive than previous
            break

        # if we've reached the end of the maze, we don't need to search more
        if (i, j) == end:
            lowest_score = score
            winning_paths |= path
            winning_path_list.append(path)
            # don't break, so we can investigate other paths that might also hit
            # the end, rather than just taking the first-best
            continue

        if not visitable(dir, i, j, score):
            continue

        dx, dy = directions4[dir]

        x, y = i + dx, j + dy
        if (x, y) not in excludes and visitable(dir, x, y, score + 1):
            #  continuing 'forward' (in whatever we were going)
            # so att that direction to the heap with a score +1
            heappush(heap, (score + 1, dir, x, y, path | {(x, y)}))

        left = (dir - 1) % 4
        if (left, i, j) not in visited:
            # add a turn candidate to the heap ukeeping the same position,
            # but with a turn)
            heappush(heap, (score + 1000, left, i, j, path))

        right = (dir + 1) % 4
        if (right, i, j) not in visited:
            # ...and the other turn candidate
            heappush(heap, (score + 1000, right, i, j, path))
    return len(winning_paths), winning_path_list


# ------------------------------------------------------------
#                             Solve
# ------------------------------------------------------------
graph = Grid(parse("16", show=0))
start = only(graph.findall("S"))
end = only(graph.findall("E"))


if __name__ == "__main__":
    print(f"{start=}")
    print(f"{end=}")
    graph.print(highlight=[start, end], block="#")
    score = bfs(graph, start, end)
    n_seats, paths = bfs_tracking_path(graph, start, end)
    print(f"{score}")
    print(f"{n_seats=} {len(paths)=} ")

#     print(f"{paths[0]}")
#     pathstrs = [graph.printstr(highlight=p, block="#") for p in paths]
#     for ps in zip(*pathstrs):
#         ps = ["".join(p) for p in ps]
#         print("   ".join(ps))
