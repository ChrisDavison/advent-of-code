from collections import defaultdict
import numpy as np
import re
from utility import (
    T,
    minmax,
    split_at,
    quantify,
    powerset,
    batched,
    sliding_window,
    first,
    last,
    nth,
    first_true,
    as_grid,
)

SAMPLE = """MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"""
DATA = open("input/04").read()


def part1(data):
    candidates = []
    horizontal = [line[i : i + 4] for line in data for i in range(len(line) - 3)]

    verticals = []
    for col in range(len(data[0])):
        for linum, line in enumerate(data[:-3]):
            verticals.append([data[linum + j][col] for j in range(4)])

    diagonal_1 = [
        [data[i + ik][j + jk] for ik, jk in zip(range(4), range(4))]
        for i in range(len(data) - 3)
        for j in range(len(data[0]) - 3)
    ]

    diagonal_2 = []
    indices = list(zip([0, -1, -2, -3], [0, 1, 2, 3]))
    for linum in range(3, len(data)):
        for colnum in range(len(data[0]) - 3):
            diagonal_2.append([data[linum + x][colnum + y] for x, y in indices])

    candidates = [*verticals, *horizontal, *diagonal_1, *diagonal_2]
    res = sum(1 for c in candidates if "".join(c) in ["XMAS", "SAMX"])
    return res


sd = [list(line) for line in DATA.splitlines()]
ss = [list(line) for line in SAMPLE.splitlines()]


print(part1(ss))
print(part1(sd))
print()


def part2(data):
    tot = 0
    want = ["SAM", "MAS"]
    for col in range(len(data[0]) - 2):
        for row in range(len(data) - 2):
            d1 = data[row][col] + data[row + 1][col + 1] + data[row + 2][col + 2]
            d2 = data[row + 2][col] + data[row + 1][col + 1] + data[row][col + 2]

            if d1 in want and d2 in want:
                tot += 1

    return tot


print(part2(ss))
print(part2(sd))
