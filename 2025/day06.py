from utility import *
import numpy as np
from functools import reduce

def pparse(filestr):
    ds = parse(filestr, atoms, show=False)[:-1]
    symbols = re.split(r"\s+", open(f"input/{filestr}").read().splitlines()[-1].strip())
    return ds, symbols

def part1(data, sym):
    out = [0] * len(data[0])
    for i, row in enumerate(data):
        for col, (s, val) in enumerate(zip(sym, row)):
            if s == "*":
                if i == 0:
                    out[col] = val
                else:
                    out[col] *= val
            else:
                out[col] += val
    print(f"part1: {chalk.red(sum(out))}")


def change_endian(values):
    pass

def part2(fname):
    lines = open("input/" + fname).read().splitlines()

    def yield_column(lines):
        symbols = re.split(r"\s+", lines[-1].strip())
        lines = lines[:-1]
        while lines[0]:
            newpos = 0
            for l in lines:
                np = l.find(' ')
                if np > newpos:
                    newpos = np
                if np == -1:
                    newpos = len(l)
                    break
            col = [l[:newpos] for l in lines]
            lines = [l[newpos+1:] for l in lines]
            sym, symbols = symbols[0], symbols[1:]
            yield col, sym

    tot = 0
    for col, s in yield_column(lines):
        t = [""] * len(col[0])
        for i in range(len(t)):
            for val in col:
                t[i] += val[i]
        t = list(map(int, t))
        if s == "*":
            val = reduce(lambda acc, x: acc * x, t[1:], t[0])
        else:
            val = sum(t)
        tot += val
    print(f"part2: {chalk.red(tot)}")

ds, dss = pparse("06s")
dd, dds = pparse("06")

part1(ds, dss)
part1(dd, dds)
print()

part2("06s")
part2("06")
