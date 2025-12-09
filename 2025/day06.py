from utility import *
from functools import reduce


def pparse(filestr):
    ds = parse(filestr, atoms, show=False)[:-1]
    symbols = re.split(r"\s+", open(f"input/{filestr}").read().splitlines()[-1].strip())
    return ds, symbols


def yield_column(lines):
    symbols = re.split(r"\s+", lines[-1].strip())
    lines = lines[:-1]
    while lines[0]:
        newpos = 0
        for l in lines:
            np = l.find(" ")
            if np > newpos:
                newpos = np
            if np == -1:
                newpos = len(l)
                break
        col = [l[:newpos] for l in lines]
        lines = [l[newpos + 1 :] for l in lines]
        sym, symbols = symbols[0], symbols[1:]
        yield col, sym


def part1(fname):
    lines = open("input/" + fname).read().splitlines()
    tot = 0
    for col, s in yield_column(lines):
        t = mapt(int, col)
        if s == "*":
            tot += reduce(lambda acc, x: acc * x, t[1:], t[0])
        else:
            tot += sum(t)
    return tot


def part2(fname):
    lines = open("input/" + fname).read().splitlines()

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
    return tot


ds, dss = pparse("06s")
dd, dds = pparse("06")


header(6, 1)
print(part1("06s"))
print(part1("06"))

header(6, 2)
print(part2("06s"))
print(part2("06"))
