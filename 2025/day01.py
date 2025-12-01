from utility import *
import numpy as np
import math

data = parse("01s", show=False)
data = parse("01", show=False)

vals = [(row[0], int(row[1:])) for row in data]

# part1
start = 50
part1 = 0
for dir, val in vals:
    if dir == 'R':
        start += val
    else:
        start -= val
    start %= 100
    if start == 0:
        part1 += 1
print(f"{part1 = }")
print()

def inc(start, value, direction):
    if direction == 'R':
        final = (start + value) % 100
        does_wrap = value > (100 - start) or start == 0
        rem = value - (100 - start)
    else:
        final = (start - value) % 100
        does_wrap = value > start and start != 0
        rem = value - start
    zeroes = 1 if final == 0 else 0
    arrow = {'L': "←", "R": "→"}[direction]

    if does_wrap:
        zeroes += 1+int(rem / 100)
        print(f"{direction}{value}\t{start:>2d} {arrow} {final:<2d} \t{zeroes} ({rem})", end=' \t')
    else:
        print(f"{direction}{value}\t{start:>2d} {arrow} {final:<2d} \t{zeroes}", end=' \t')

    return final, zeroes

# part2
print("dir, S, E, 0, rem")
start = 50
part2 = 0
for dir, val in vals:
    sym = '+' if dir == 'R' else '-'
    final, wraps = inc(start, val, dir)
    start = final
    part2 += wraps
    print(part2)
print(f"{part2 = }")
