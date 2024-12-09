from utility import *
import sys
from dataclasses import dataclass
from simple_chalk import chalk
import itertools as it

year = 2024
day = 9
prefix = f"{year}.{day:02d}."
DEBUG = False


@dataclass
class Segment:
    num: int
    id: int
    size: int


@dataclass
class Gap:
    size: int


def parse(line):
    segments = []
    id = 0
    for i, ch in enumerate(line):
        ch = int(ch)
        if i % 2 == 0:
            segments.append(Segment(i, id, ch))
            id += 1
        else:
            segments.append(Gap(ch))
    return segments


def part1(data):
    segments = parse(data)
    segment_str = ""
    for s in segments:
        if isinstance(s, Gap):
            segment_str += s.size * "."
        else:
            segment_str += s.size * str(s.id)

    last_digit = None
    idx_last_digit = len(segment_str) - 1
    print(f"{idx_last_digit = :}")

    n_dots = sum(1 for ch in segment_str if ch == ".")
    n_after_dots = sum(1 for ch in it.takewhile(lambda x: x == ".", segment_str[::-1]))
    if DEBUG:
        print(f"{n_dots, n_after_dots = :} => {n_dots - n_after_dots}")
        print("".join([s for s in segment_str]))

    for i in range(len(segment_str)):
        ch = segment_str[i]
        if ch != ".":
            continue
        idx_last_digit = len(segment_str.rstrip(".")) - 1
        last_digit = segment_str[idx_last_digit]

        spaces = list(" " * len(segment_str))
        spaces[i] = "v"
        spaces[idx_last_digit] = "v"
        if idx_last_digit < i:
            break
        segment_str = (
            segment_str[:i]
            + last_digit
            + segment_str[i + 1 : idx_last_digit]
            + "."
            + segment_str[idx_last_digit + 1 :]
        )
        # segment_str[idx_last_digit] = "."
        idx_last_digit -= 1
        if DEBUG:
            print("".join(spaces))
            print(
                "".join(
                    [
                        s if j not in [i, idx_last_digit + 1] else chalk.red(s)
                        for j, s in enumerate(segment_str)
                    ]
                ),
            )
    if DEBUG:
        print("".join(segment_str))
    checksum = 0
    for i, val in enumerate(segment_str):
        if val == ".":
            continue
        checksum += i * int(val)
    print(checksum)


def part2(data):
    res = None
    print(f"{prefix}2 -- {res}")


DEBUG = False
if len(sys.argv) > 1:
    print(f"{len(sys.argv) = :}")
    data = open(sys.argv[1]).read().strip()
else:
    data = sys.stdin.read().strip()
part1(data)
