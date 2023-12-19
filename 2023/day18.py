from utility import *

SAMPLE = """R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"""

DATA = Path("input/18").read_text()


def lineparser_1(line):
    direc = {"R": (0, 1), "D": (1, 0), "L": (0, -1), "U": (-1, 0)}
    d, n, _ = line.split(" ")  # dont care about colour
    dr, dc = direc[d]
    return dr, dc, int(n)


def lineparser_2(line):
    direc = {"R": (0, 1), "D": (1, 0), "L": (0, -1), "U": (-1, 0)}
    _, _, x = line.split(" ")
    x = x[2:-1]  # Trim (# and )
    dr, dc = direc["RDLU"[int(x[-1])]]  # take last as direction index
    n = int(x[:-1], 16)  # take first 5 as length
    return dr, dc, int(n)


def p(data, lineparser):
    points = [(0, 0)]
    pathlen = 0
    for line in data.splitlines():
        dr, dc, n = lineparser(line)
        r, c = points[-1]
        pathlen += n
        points.append((r + dr * n, c + dc * n))
    return points, pathlen


def shoelace(coords, pathlen):
    # ...and pick's theorem
    # area = 1/2 |sum (xi * (yi+1 - yi-1))|
    # shoelace is the boundary
    # picks is the insert region
    # A = i + boundary/2 + 1
    # so i = A - b/2 + 1
    # so area + boundary = i + b
    area = 0
    for i in range(len(coords)):
        xi = coords[i][0]
        yiprev = coords[i - 1][1]
        yinext = coords[(i + 1) % len(coords)][1]
        area += coords[i][0] * (yiprev - yinext)
    area = abs(area) / 2
    i = area - pathlen // 2 + 1
    return int(i + pathlen)


timer(reset=True)
res = shoelace(*p(DATA, lineparser_1))
timer(f"Part 1: {res}")
pyperclip.copy(res)

timer(reset=True)
res = shoelace(*p(DATA, lineparser_2))
timer(f"Part 2: {res}")
pyperclip.copy(res)
