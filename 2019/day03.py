#!/usr/bin/env python3
from prelude import *
import numpy as np

sample = """
R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83
""".strip()

sample2 = """
R8,U5,L5,D3
U7,R6,D4,L4
""".strip()

sample3 = """
R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
""".strip()



def get_path(instructions):
    origin = np.complex(0, 0)
    path = set()
    total = 0
    for ins in instructions:
        dir = ins[0]
        num = int(ins[1:])
        step = np.complex(0, 0)
        if dir == 'R':
            step = np.complex(1, 0)
        elif dir == "L":
            step = np.complex(-1, 0)
        elif dir == "U":
            step = np.complex(0, 1)
        else:
            step = np.complex(0, -1)
        for i in range(num):
            total += 1
            origin += step
            path.add((total, origin))
    return path


def walk_paths(data):
    seens = []
    for line in data:
        seens.append(get_path(line.split(",")))
    return seens


def part1(paths):
    points = [[point for steps, point in ls] for ls in paths]
    intersect = set(points[0])
    for s in points[1:]:
        intersect &= set(s)
    manh = min([int(np.abs(c.real) + np.abs(c.imag)) for c in intersect])
    print(f"2019 3.1 -> {manh}")
    clip.copy(manh)


def part2(paths):
    points = [[point for steps, point in ls] for ls in paths]
    intersect = set(points[0])
    for s in points[1:]:
        intersect &= set(s)
    min_total = None
    for i in intersect:
        i_total = 0
        for seen in paths:
            for s in seen:
                if s[1] == i:
                    i_total += s[0]
        if not min_total or i_total < min_total:
            min_total = i_total
    print(f"2019 3.2 -> {min_total}")
    clip.copy(min_total)


data = [l for l in open('input/03').read().splitlines()]
paths = walk_paths(data)
part1(paths)
part2(paths)
