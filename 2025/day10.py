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
        triggers = []
        for chunk in rest[: vstart - 1].split(" "):
            z = ["0"] * len(dstr)
            for col in chunk[1:-1].split(","):
                z[int(col)] = "1"
            triggers.append(int("".join(z), 2))
        parsed.append((diagram, tuple(triggers), voltages))
    return parsed


def apply(nsw):
    out = [s for n, s in nsw for _ in range(n) ]
    return reduce(lambda x, acc: x ^ acc, out, 0)

def step(target, switches):
    m = None
    for v in range(1, 2**len(switches)):
        mask = [int(c) for c in bin(v)[2:].rjust(len(switches), '0')]
        fin = apply(zip(mask, switches))
        if fin != target:
            continue
        if not m or sum(mask) < m:
            m = sum(mask)
    return m


data = pparse("10s")
data = pparse("10")

result = 0
for diagram, switches, _ in data:
    steps = step(diagram, switches)
    result += steps

print(result)
