from utility import *
from heapq import heappop, heappush
import time
import os


def bfs_tracking_path(graph, start, end, wallskip=None):
    excludes = set(graph.findall("#"))
    heap = [(0, 0, *start, [start])]  # score, direction, position, path_points
    visited = {}
    lowest_score = None
    winning_path_list = []  # keep track of all the best paths
    winning_paths = set()  # keep track of seats that are on any best path

    if wallskip:
        excludes -= {wallskip}

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
            winning_paths |= set(path)
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
            heappush(heap, (score + 1, dir, x, y, path + [(x, y)]))

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


@cache
def dist(a, b):
    return taxi_distance(a, b)


@cache
def within_taxi_distance(n, min=0):
    points = []
    for i in range(n + 1):
        for j in range(n + 1):
            d = dist((i, j), (0, 0))
            if min <= d <= n:
                points.append((i, j))
                points.append((i, -j))
                points.append((-i, -j))
                points.append((-i, j))
    return points


def path_neighbour_walls(route, distance=2):
    offsets = within_taxi_distance(distance, 2)
    points = dict()
    # route is now a dict of (x, y): index
    # so that I can easily calculate the number of spaces skipped
    for (x, y), i in route.items():
        for dx, dy in offsets:
            nx = x + dx
            ny = y + dy
            if (nx, ny) not in route:
                continue
            ni = route[(nx, ny)]  # next i
            # print(f"{ni=} {i=} ")
            d = abs(dx) + abs(dy)
            # If the point is further along the track than our current position,
            # yield it
            delta = ni - i
            if delta > (i + d):
                points[(x, y, nx, ny, ni - i - 2)] = 1
    return points.keys()


def run(filename, distance, debug=False):
    grid = Grid(parse(filename, show=0))
    start = only(grid.findall("S"))
    end = only(grid.findall("E"))
    # grid.print(highlight=(start, end), block="#")
    _, routes = bfs_tracking_path(grid, start, end)
    print(f"BFS path generated -- length {len(routes[0])}")
    if set(routes[0]) == intersection(routes):
        routes = [routes[0]]
    else:
        print("...multiple routes?")

    for route in routes:
        route = {r: i for i, r in enumerate(route)}
        cheats = path_neighbour_walls(route, distance=distance)
        hundred = 0
        deltas = [delta for x, y, nx, ny, delta in cheats]
        cdeltas = Counter(deltas)
        for k in sorted(cdeltas.keys()):
            if k >= 50:
                print(f"{k}, {cdeltas[k]}")
            if k >= 100:
                hundred += cdeltas[k]
        hundred = sum(v for k, v in cdeltas.items() if k >= 100)
        print(f"cheats over 100ps = {hundred}")
        break


def part1(filename, debug=False):
    run(filename, 2, debug=debug)


def part2(filename, debug=False):
    run(filename, 20, debug=debug)


# part1("20s")
part1("20")
# part2("20s")
part2("20")
# run("20s", 20, False)
