from utility import *


down = Point2D(0, 1)
left = Point2D(-1, 0)
right = Point2D(1, 0)


def pparse(filename):
    splitters = set()
    start = None
    for i, row in enumerate(open(filename).read().splitlines()):
        for j, sym in enumerate(row):
            if sym == '^':
                splitters.add(Point2D(j, i))
            if sym == 'S':
                start = Point2D(j, i)
    assert start is not None
    return splitters, start

def part1(filename):
    splitters, start = pparse(filename)
    beams = {start - Point2D(0, -1)}
    end_row = max(p.y for p in splitters) + 1
    print(p1r(beams, splitters, end_row))

def p1r(beams, splitters, final_row, splits=0):
    if any(b.y >= final_row for b in beams):
        return splits

    next_beams = {b + down for b in beams}
    nb = set()
    for b in next_beams:
        if b not in splitters:
            nb.add(b)
            continue
        l = b + left
        r = b + right
        splits += 1
        if l not in nb:
            nb.add(l)
        if r not in nb:
            nb.add(r)
    return p1r(nb, splitters, final_row, splits)

def part2(filename):
    splitters, start = pparse(filename)
    maxrow = max(s.y for s in splitters)
    res = part2r(start+down, splitters, endrow=maxrow)
    print(f"{res = }")

def part2r(beam, splitters, endrow=None):
    if beam.y >= endrow:
        return 1

    nb = beam + down
    next_splitters = {s for s in splitters if s.y >= nb.y}
    print(f"{len(splitters) = }")
    if nb in splitters:
        l = nb + left
        r = nb + right
        return part2r(l, next_splitters, endrow=endrow) + part2r(r, next_splitters, endrow=endrow)
    return part2r(nb, next_splitters, endrow=endrow)


part1("input/07s")
part1("input/07")

part2("input/07s")
part2("input/07")
