from collections import defaultdict
import re
from utility import *
import sys

year = YEAR
day = DAY
prefix = f"{year}.{day:02d}."

DATA = sys.stdin().read()

data = []
for line in DATA.splitlines():
    data.append(line)


def part1(data):
    res = None
    print(f"{prefix}1 -- {res}")


def part2(data):
    res = None
    print(f"{prefix}2 -- {res}")


part1(data)
