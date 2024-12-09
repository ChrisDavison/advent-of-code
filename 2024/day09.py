from utility import *
import sys
from dataclasses import dataclass
from simple_chalk import chalk
import itertools as it
import numpy as np
from collections import namedtuple

year = 2024
day = 9
prefix = f"{year}.{day:02d}."
DEBUG = False


def part1(data):
    ss = []
    gg = []
    pos = 0
    s_idx = 0
    for i, ch in enumerate(data):
        ch = int(ch)
        if ch == 0:
            continue
        if i % 2 == 0:
            for j in range(ch):
                ss.append((pos, ch, s_idx))
                pos += 1
            s_idx += 1
        else:
            for j in range(ch):
                gg.append(pos)
                pos += 1

    def displaystr(ss, gg, last_change=None):
        maxpos = 0
        for s in ss:
            maxpos = max(maxpos, s[0])
        for g in gg:
            maxpos = max(maxpos, g)
        disp = np.zeros(maxpos + 1, dtype=int)
        for s in ss:
            disp[s[0]] = s[2]
        for g in gg:
            disp[g] = -1

        stuff = disp.astype(str).tolist()
        if last_change:
            a, b = last_change
            stuff[a] = chalk.red("^")
            stuff[b] = chalk.red(stuff[b])
        output = "".join(stuff).replace("-1", ".")
        return output

    hole_idx = 0
    segment_idx = len(ss) - 1
    if DEBUG:
        print(displaystr(ss, gg))
    while True:
        if hole_idx >= len(gg) or segment_idx < 0:
            break
        if gg[hole_idx] > ss[segment_idx][0]:
            break
        gg[hole_idx], ss[segment_idx] = (
            ss[segment_idx][0],
            (gg[hole_idx], ss[segment_idx][1], ss[segment_idx][2]),
        )
        if DEBUG:
            print(displaystr(ss, gg, last_change=(gg[hole_idx], ss[segment_idx][0])))
        hole_idx += 1
        segment_idx -= 1

    # print(f"{ss = :}")
    ii = np.array(ss)[:, 0]
    vv = np.array(ss)[:, 2]
    checksum = np.sum(ii * vv)
    print("part1", checksum)


DEBUG = False
if len(sys.argv) > 1:
    data = open(sys.argv[1]).read().strip()
else:
    data = sys.stdin.read().strip()
part1(data)
