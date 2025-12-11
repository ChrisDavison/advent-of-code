from functools import lru_cache, partial, reduce
from utility import *
from itertools import product
from z3 import IntVector, Optimize, Sum, sat


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
    # TODO review Integer Programming and think how I would implement it myself
    switches = [bitmask_to_indices(s) for s in switches]
    n = len(voltages)
    X = IntVector('x', len(switches))

    s = Optimize()
    for k in range(len(switches)):
        s.add([x >= 0 for x in X])

    A = []
    for button in switches:
        row = [0 for _ in range(n)]
        for w in button:
            row[w] = 1
        A.append(row)

    for i in range(n):
        s.add(Sum(X[k]*A[k][i] for k in range(len(switches))) == voltages[i])
    s.minimize(Sum(X))

    # Check for satisfiability and get the model
    if s.check() == sat:
        model = s.model()
        return sum(model[k].as_long() for k in model)
    else:
        print("No solution found.")
        return 0

def p1(filename):
    data = pparse(filename)
    col = chalk.red if "s" not in filename else chalk.green
    result = 0
    for diagram, switches, _ in data:
        steps = part1(diagram, switches)
        result += steps
    print(col(f"part1 {result}"))

def p2(filename):
    data = pparse(filename)
    col = chalk.red if "s" not in filename else chalk.green
    result = 0
    for _, switches, voltages in data:
        result += part2(switches, voltages)
        # break
    print(col(f"part2 {result}"))

p1("10s")
p1("10")

p2("10s")
p2("10")
