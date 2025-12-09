from utility import *
from itertools import permutations

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

def part2(data):
    pass

part1(ds)
part1(dd)
