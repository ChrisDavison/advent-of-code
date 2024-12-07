import sys
import functools as ft
import re
import math as m
from typing import List

year = 2024
day = 7
prefix = f"{year}.{day:02d}."

calibrations = []
for line in sys.stdin.read().splitlines():
    values = list(int(v) for v in re.findall(r"[0-9]+", line))
    calibrations.append((values[0], list(values[1::])))


def run(data):
    tot = 0
    tot2 = 0
    for answer, numbers in data:
        if solve(answer, numbers):
            tot += answer
        elif solve(answer, numbers, True):
            tot2 += answer

    print(f"{prefix}1 -- {tot}")
    print(f"{prefix}2 -- {tot+tot2}")


@ft.cache
def scale(num: int) -> int:
    return int(m.log10(num)) + 1


def concat(a: int, b: int) -> int:
    return a * 10 ** scale(b) + b


def solve(target: int, ll: List[int], part2=False) -> bool:
    if len(ll) == 1:
        return target == ll[0]

    first, second, rest = ll[0], ll[1], ll[2:]
    if solve(target, [first + second] + rest, part2):
        return True
    if solve(target, [first * second] + rest, part2):
        return True
    if part2 and solve(target, [concat(first, second)] + rest, part2):
        return True
    return False


run(calibrations)
