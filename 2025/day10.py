from functools import lru_cache, partial, reduce
from utility import *
from pprint import pprint as pp
from itertools import product


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

@lru_cache
def apply2(mask, sw, target):
    out = [0] * len(target)
    for m, s in zip(mask, sw):
        for col in s:
            out[col] += m
            if out[col] > target[col]:
                return tuple(out)
    return tuple(out)

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


def part2(switches, voltages):
    switch_max = [0] * len(switches)
    switches = tuple([bitmask_to_indices(sw) for sw in switches])
    for i, sw in enumerate(switches):
        switch_max[i] = min([voltages[j] for j, s in enumerate(sw)
                         if voltages[j] and s])

    presses = sum(switch_max)
    print("N variants", reduce(lambda x, acc: x * acc, switch_max, 1))
    # CANNOT DO THIS
    # have 1E18 variants for the first entry...and ~200 entries
    for variant in product(*[list(range(n+1)) for n in switch_max]):
        sv = sum(variant)
        if sv > presses:
            continue
        if sv < max(voltages):
            continue
        res = apply2(variant, switches, voltages)
        if res == voltages:
            presses = min(presses, sv)

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
    steps = part2(switches, voltages)
    result += steps
print(result)
