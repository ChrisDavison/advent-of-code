from utility import *
from itertools import permutations, combinations
from shapely import Polygon, box

ds = [Point2D(p[0], p[1]) for p in parse("09s", ints, show=False)]
dd = [Point2D(p[0], p[1]) for p in parse("09", ints, show=False)]

def area(p1, p2):
    d = p2 - p1
    a = (abs(d.x)+1) * (abs(d.y)+1)
    # print(f"{p1.x:2d},{p1.y:2d}...{p2.x, p2.y}\t{d}\t{a}")
    return a

def part1(data):
    a = 0
    for p1, p2 in combinations(data, 2):
        ar = area(p1, p2)
        a = max(ar, a)
    print(f"part1: {chalk.red(a)}")


def find_squares(data):
    def is_valid(xy):
        a, b = xy
        if a.x == b.x or a.y == b.y:
            return True
        return True

    max_x = max(p2.x - p1.x for p1, p2 in combinations(data, 2))
    max_y = max(p2.y - p1.y for p1, p2 in combinations(data, 2))
    print(f"{max_x, max_y = } {max_x * max_y}")
    corners = sorted(combinations(data, 2), key=lambda x: area(*x), reverse=True)
    valid = filter(is_valid, corners)
    n = next(valid)
    print(f"{n = }")
    return area(*n)

def part2(data):
    # TODO review this and figure out my own 'polygon' and 'contains' methods
    def area(edge) -> int:
        ((x1, y1), (x2, y2)) = edge
        return (abs(x2 - x1) + 1) * (abs(y2 - y1) + 1)

    data = [tuple((d.x, d.y)) for d in data]
    polygon = Polygon(data)

    for edge in sorted(combinations(data, 2), key=area, reverse=True):
        (x1, y1), (x2, y2) = edge
        if polygon.contains(box(x1, y1, x2, y2)):
            print(area(edge))
            return area(edge)

part1(ds)
part1(dd)

part2(ds)
part2(dd)
