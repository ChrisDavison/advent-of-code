from collections import namedtuple
from functools import lru_cache, partial, reduce
from utility import *
from numpy import array, zeros_like
from pprint import pprint as pp


pp = partial(pp, indent=2)


def pparse(filename):
    parsed = []
    m = {".": "0", "#": "1"}
    for line in open("input/" + filename).read().splitlines():
        dstr, rest = line.split(" ", 1)
        dstr = dstr[1:-1]
        diagram = int("".join([m[c] for c in dstr]), 2)
        vstart = rest.find("{")
        voltages = tuple([int(v) for v in rest[vstart + 1 : -1].split(",")])
        switches = []
        for chunk in rest[: vstart - 1].split(" "):
            z = ["0"] * len(dstr)
            for col in chunk[1:-1].split(","):
                z[int(col)] = "1"
            switches.append([int(zz) for zz in z])
        parsed.append((diagram, tuple(switches), voltages))
    return parsed


def apply(mask, sw):
    out = [s for n, s in zip(mask, sw) for _ in range(n)]
    return reduce(lambda x, acc: x ^ acc, out, 0)

def apply2(mask, sw, width):
    out = [0] * width
    for m, s in zip(mask, sw):
        for col in s:
            out[col] += m
    return out

def bitmask_to_indices(bm):
    indices = tuple([i for i, b in enumerate(bm) if b])
    return indices


def part1(target, switches):
    m = 0
    switches = [int("".join(str(s) for s in sw), 2) for sw in switches]
    for v in range(1, 2 ** len(switches)):
        mask = [int(c) for c in bin(v)[2:].rjust(len(switches), "0")]
        fin = apply(mask, switches)
        if fin != target:
            continue
        if not m or sum(mask) < m:
            m = sum(mask)
    return m


def yield_permutations(switch_max):
    fields_to_consider = [i for i, m in switch_max if m > 0]


def part2(switches, voltages):
    switch_max = [0] * len(switches)
    switches = [bitmask_to_indices(sw) for sw in switches]
    for i, sw in enumerate(switches):
        mp = None
        for s in sw:
            if not s or not voltages[s]:
                continue
            mult = int(voltages[s] / s)
            if not mp:
                mp = mult
            else:
                mp = min(mult, mp)
        switch_max[i] = mp

    N = len(voltages)
    print("apply2", apply2([0] * (N-1) + [1], switches, N))
    while any(s > 0 for s in switch_max):
        # TODO fix
        # this works partially, but need to run through _ALL_ variants of every column
        # e.g. [2, 2, 5]
        #      [1, 2, 5]
        #      [0, 2, 5]
        #      [2, 1, 5]
        #      [2, 0, 5]
        #      [1, 1, 5]
        #      [1, 0, 5]
        #      [0, 0, 5]
        # .... and so on
        print(f"{switch_max = }")
        res = apply2(switch_max, switches, N)
        if res == voltages:
            presses = sum(switch_max)
        first_non_zero = [i for i, v in enumerate(switch_max) if v > 0][0]
        switch_max[first_non_zero] -= 1

    return presses


data = pparse("10s")
data = pparse("10")

result = 0
for diagram, switches, _ in data:
    steps = part1(diagram, switches)
    result += steps
print(result)

print("-" * 40)

data = pparse("10s")
# data = pparse("10")
result = 0
for _, switches, voltages in data:
    print(f"{switches = }")
    steps = part2(switches, voltages)
    result += steps
    break
print(result)
