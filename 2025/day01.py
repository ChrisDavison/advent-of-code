from utility import *
import numpy as np
import math

data = parse("01s")
# #data = parse("01")

vals = [(row[0], int(row[1:])) for row in data]
vals = [(r, v if r == 'R' else -v) for r, v in vals]

# part1
start = 50
part1 = 0
for dir, val in vals:
    start += val
    start %= 100
    if start == 0:
        part1 += 1
print(f"{part1 = }")
print("░" * 80)
