from collections import defaultdict
import re
import pyperclip
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

SAMPLE = """xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"""
SAMPLE2 = (
    """xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"""
)
DATA = open("input/03").read()
source = DATA

P = []
m = re.findall(r"mul\((\d+),(\d+)\)", source)
s = 0
do = True
for group in m:
    x, y = int(group[0]), int(group[1])
    s += x * y
print(f"Part 1: {s}")

m = re.findall(r"(don't\(\)|do\(\)|mul\((\d+),(\d+)\))", source)
s = 0
do = True
for group in m:
    match group[0]:
        case "don't()":
            do = False
        case "do()":
            do = True
        case default:
            if do:
                x, y = int(group[1]), int(group[2])
                s += x * y
print(f"Part 2: {s}")
