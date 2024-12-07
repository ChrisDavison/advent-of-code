from utility import ints
import sys


def check(levels):
    # print(f"{d = :}")
    samesign = all([x > 0 for x in d]) or all([x < 0 for x in d])
    if all([abs(x) in [1, 2, 3] for x in d]) and samesign:
        return True


data = sys.stdin().readlines()

P = []
s = 0
for line in data:
    i = ints(line)
    d = [b - a for a, b in zip(i, i[1:])]
    if check(d):
        s += 1
print(f"Part 1: {s}")


P = []
s = 0
for line in data:
    i = ints(line)
    for j in range(len(i)):
        subset = i[:j] + i[j + 1 :]
        print(f"{subset = :}")
        d = [b - a for a, b in zip(subset, subset[1:])]
        if check(d):
            s += 1
            break
print(f"Part 2: {s}")
