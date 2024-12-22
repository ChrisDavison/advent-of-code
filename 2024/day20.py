from utility import *
from heapq import heappop, heappush
from collections import namedtuple

State = namedtuple("State", "fn grid start end route")


@cache
def parse_file(filename) -> State:
    grid = Grid(parse(filename, show=0))
    start = only(grid.findall("S"))
    end = only(grid.findall("E"))
    routes = bfs_tracking_path(grid, start, end)
    print(f"Track length: {len(routes[0])}")
    return State(filename, grid, start, end, routes[0])


def dijkstra(grid, end_char="E") -> Dict[Point, int]:
    """All-paths distances from each point to the end square on the grid: {(x, y): distance}."""
    end = the(grid.findall(end_char))
    D = {end: 0}
    Q = [end]
    while Q:
        p = Q.pop()
        for p2 in grid.neighbors(p):
            if grid[p2] != "#" and p2 not in D:
                Q.append(p2)
                D[p2] = D[p] + 1
    return D


def big_cheats(
    racetrack, lower_bound=1, radius=20
) -> Iterable[Tuple[Point, Point, int]]:
    """All ways of cheating by taking up to `radius` steps through walls and back to the track."""
    D = dijkstra(racetrack, "E")
    return (
        (p1, p2, t)
        for p1 in D
        for p2 in neighborhood(p1, radius)
        if p2 in D and (t := D[p1] - D[p2] - taxi_distance(p1, p2)) >= lower_bound
    )


def neighborhood(point, radius) -> List[Point]:
    """All points within `radius` of `point` (using taxi distance)."""
    (x, y) = point
    return [
        (x + dx, y + dy)
        for dx in range(-radius, radius + 1)
        for dy in range(-(radius - abs(dx)), radius - abs(dx) + 1)
    ]


def bfs_tracking_path(graph, start, end, wallskip=None):
    excludes = set(graph.findall("#"))
    heap = [(0, 0, *start, [start])]  # score, direction, position, path_points
    visited = {}
    lowest_score = None
    winning_path_list = []  # keep track of all the best paths

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
    return winning_path_list


def path_neighbour_walls(route, distance=2):
    points = set()
    # route is now a dict of (x, y): index
    # so that I can easily calculate the number of spaces skipped
    for (x, y), i in route.items():
        candidates = [
            (x + dx, y + dy)
            for dx in range(-distance, distance + 1)
            for dy in range(-(distance - abs(dx)), (distance - abs(dx)) + 1)
        ]
        for nx, ny in candidates:
            ni = route.get((nx, ny), -1)
            # If the point is further along the track than our current position,
            # yield it
            if ni > i:
                points.add((x, y, nx, ny, ni - i - 2))
    return points


def run(route, distance, debug=False):
    route = {r: i for i, r in enumerate(route)}
    cheats = path_neighbour_walls(route, distance=distance)
    hundred = 0
    deltas = [delta for x, y, nx, ny, delta in cheats]
    hundred = len(list(filter(lambda d: d >= 100, deltas)))
    # c = Counter(deltas)
    # for k in sorted(c.keys()):
    #     if k > 50:
    #         print(k, c[k])
    print(f"cheats over 100ps = {hundred}")


def part1(state, debug=False):
    print(state.fn)
    run(state.route, 2, debug=debug)


def part2(state, debug=False):
    print(state.fn)
    run(state.route, 20, debug=debug)


if __name__ == "__main__":
    # part1(parse_file("20s"))
    # part1(parse_file("20"))

    # part2(parse_file("20s"))
    part2(parse_file("20"))
    # run("20s", 20, False)
