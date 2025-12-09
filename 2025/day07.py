from utility import *

def pparse(filename):
    splitters = set()
    start = None
    for i, row in enumerate(open(filename).read().splitlines()) :
        for j, sym in enumerate(row):
            if sym == '^':
                splitters.add(Point2D(j, i))
            if sym == 'S':
                start = Point2D(j, i)
    assert start is not None
    return splitters, start

def part1(filename):
    splitters, start = pparse(filename)
    down = Point2D(0, -1)
    left = Point2D(-1, 0)
    right = Point2D(1, 0)
    beam_ends = {start - down}
    end_row = max(p.y for p in splitters) + 1
    splits = 0
    finished = False
    while not finished:
        new_beams = set()
        for beam in beam_ends:
            nb = beam - down
            if nb.y >= end_row:
                finished = True
                break
            if nb in beam_ends:
                continue
            elif nb not in splitters:
                # straight down continuation
                new_beams.add(nb)
            else:
                splits += 1
                l = nb + left
                r = nb + right
                if l not in splitters and l not in beam_ends:
                    new_beams.add(l)
                if r not in splitters and r not in beam_ends:
                    new_beams.add(r)
        beam_ends = new_beams
        # print(new_beams)
        # input()
    print(splits)

def part2(data):
    pass

part1("input/07s")
part1("input/07")

