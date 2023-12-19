from utility import *

SAMPLE = """7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"""

SAMPLE2 = """.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."""

DATA = Path("input/10").read_text()


@timed
def part1(data=SAMPLE):
    sloc, connections, dim = parser(data)
    p1 = walk_from(sloc, connections)
    # p2 = walk_from(connections[sloc][1], connections, set([sloc]))
    return (len(p1) + 1) // 2


def walk_from(start, points):
    current = start
    chain = set([start])
    chain.add(current)
    chainl = [start]
    while True:
        try:
            nx = next(p for p in points[current] if p not in chain)
            if nx == start:
                break
            if not nx:
                break
            chain.add(current)
            chainl.append(current)
            current = nx
        except StopIteration:
            break
    chain.add(current)
    chainl.append(current)
    return chainl


def visualise(data, end="\n"):
    if isinstance(data, list):
        data = "\n".join("".join(thing for thing in row) for row in data)
    replaces = [
        (".", "#"),
        ("F", "┏"),
        ("J", "┛"),
        ("7", "┓"),
        ("L", "┗"),
        ("-", "━"),
        ("|", "┃"),
        ("S", "▒"),
        ("O", "#"),
        ("I", "I"),
    ]
    for r in replaces:
        data = data.replace(*r)
    print(data, end=end)


def shoelace(points):
    area = 0

    for i in range(len(points)):
        area += X[i] * Y[i + 1] - Y[i] * X[i + 1]

    return abs(area) / 2


@timed
def part2(data=SAMPLE):
    sloc, connections, dim = parser(data)
    chain = walk_from(sloc, connections)

    # find the area enclosed by the chain of points
    dg = [[c for c in line] for line in data.splitlines()]

    inside_squares = 0
    for y, line in enumerate(dg):
        is_in, last_f_or_l = False, None
        for x, c in enumerate(line):
            if Point2D(x, y) in chain:
                if c == "|" or c == "S":
                    is_in = not is_in
                elif c in "FL":
                    last_f_or_l = c  # keep track of if we were F or L
                elif (last_f_or_l == "F" and c == "J") or (
                    last_f_or_l == "L" and c == "7"
                ):
                    is_in = not is_in
            else:
                inside_squares += is_in
    return inside_squares


def parser(data):
    s = Point2D(0, 0)
    connections = dict()
    dim = [0, 0]
    for y, line in enumerate(data.splitlines()):
        dim[0] = max(y, dim[0])
        for x, c in enumerate(line):
            dim[1] = max(x, dim[1])
            p = Point2D(x, y, data="c")

            match c:
                case "S":
                    s = p
                    connections[p] = []
                # The following is where each point connects to
                case "L":
                    connections[p] = (p.up(), p.right())
                case "J":
                    connections[p] = (p.up(), p.left())
                case "F":
                    connections[p] = (p.down(), p.right())
                case "7":
                    connections[p] = (p.down(), p.left())
                case "|":
                    connections[p] = (p.down(), p.up())
                case "-":
                    connections[p] = (p.left(), p.right())

    for sur in s.surrounding():
        if sur not in connections:  # one of the non-connected directions
            continue
        for p in connections[sur]:
            # if any of the points surrounding s touch s,
            # make s point back to them
            if p == s:
                connections[s].append(sur)
    return s, connections, (dim[0] + 1, dim[1] + 1)


print("-" * 80)
print(datetime.datetime.now().strftime("@ %H:%M"))
part1(DATA)
print()
part2(DATA)
