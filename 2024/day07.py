import functools as ft
import math as m
import re
from argparse import ArgumentParser
from pathlib import Path
from typing import List


def run(data):
    tot = 0
    tot2 = 0
    for answer, numbers in data:
        if solve(answer, numbers):
            tot += answer
        elif solve(answer, numbers, True):
            tot2 += answer

    print(f"1 -- {tot}")
    print(f"2 -- {tot+tot2}")


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


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("-d", "--debug", action="store_true", default=False)
    parser.add_argument("file", type=Path, nargs=1)
    args = parser.parse_args()

    calibrations = []
    for line in args.file[0].read_text().splitlines():
        values = list(int(v) for v in re.findall(r"[0-9]+", line))
        calibrations.append((values[0], list(values[1::])))

    run(calibrations)
