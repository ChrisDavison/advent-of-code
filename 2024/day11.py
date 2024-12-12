from utility import ints, mapl
from collections import Counter
from pathlib import Path
from math import log10
from functools import cache
from argparse import ArgumentParser


@cache
def n_digits(d):
    return int(log10(d) + 1)


@cache
def split_stone(d):
    s = str(d)
    n = int(n_digits(d) / 2)
    return int(s[:n]), int(s[n:])


@cache
def even_digits(d):
    return n_digits(stone) % 2 == 0


@cache
def transform(stone):
    if stone == 0:
        return [1]
    elif even_digits(stone):  # even number of digits
        return split_stone(stone)
    else:
        return [stone * 2024]


if __name__ == "__main__":
    parser = ArgumentParser()
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("-s", "--sample", action="store_true")
    mode.add_argument("-d", "--data", action="store_true")
    parser.add_argument("-n", default=25, type=int)
    parser.add_argument("--debug", action="store_true", default=False)
    parser.add_argument("nums", nargs="*")
    pargs = parser.parse_args()

    if pargs.sample:
        nums = ints(Path("input/11s").read_text().strip())
    elif pargs.data:
        nums = ints(Path("input/11").read_text().strip())
    elif pargs.nums:
        nums = mapl(int, pargs.nums)
    else:
        nums = ints(input("ints: "))

    pos = 0
    n = pargs.n

    numd = Counter(nums)
    newd = Counter()

    for i in range(n):
        newd.clear()
        for stone, count in numd.items():
            for ns in transform(stone):
                newd[ns] += count
        numd = newd.copy()
    print(sum(numd.values()))
