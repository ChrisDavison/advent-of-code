from collections import defaultdict
import re
from utility import *

SAMPLE = """7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"""
DATA = open("input/02").readlines()


def check(levels):
    # print(f"{d = :}")
    samesign = all([x > 0 for x in d]) or all([x < 0 for x in d])
    if all([abs(x) in [1, 2, 3] for x in d]) and samesign:
        return True


P = []
s = 0
source = DATA
# source = SAMPLE.splitlines()
for line in source:
    i = ints(line)
    d = [b - a for a, b in zip(i, i[1:])]
    if check(d):
        s += 1
print(f"Part 1: {s}")


P = []
s = 0
source = SAMPLE.splitlines()
source = DATA
for line in source:
    i = ints(line)
    for j in range(len(i)):
        subset = i[:j] + i[j + 1 :]
        d = [b - a for a, b in zip(subset, subset[1:])]
        if check(d):
            s += 1
            break
print(f"Part 2: {s}")
