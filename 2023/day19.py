from collections import defaultdict
import re
from utility import T, minmax, split_at, quantify, powerset, batched, sliding_window, first, last, nth, first_true, as_grid

SAMPLE = """ """
DATA = open("input/19").readlines()

P = []
for line in SAMPLE.splitlines():
    P.append(line)

print(f"Part 1: {0}")

