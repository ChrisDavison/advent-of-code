import sys
from dataclasses import dataclass
from simple_chalk import chalk
import numpy as np

year = 2024
day = 9
prefix = f"{year}.{day:02d}."
DEBUG = False


@dataclass
class Segment:
    pos: int
    n: int
    id: int


@dataclass
class Gap:
    pos: int
    n: int


def displaystr(ss, gg, last_change=None):
    maxpos = 0
    for s in ss:
        maxpos = max(maxpos, s[0])
    for g in gg:
        maxpos = max(maxpos, g[0])
    disp = np.zeros(maxpos + 1, dtype=int)
    for s in ss:
        disp[s[0]] = s[2]
    for g in gg:
        disp[g[0]] = -1

    stuff = disp.astype(str).tolist()
    if last_change:
        a, b = last_change
        stuff[a] = chalk.red("^")
        stuff[b] = chalk.red(stuff[b])
    output = "".join(stuff).replace("-1", ".")
    return output


def checksum(segments):
    ii = np.array(segments)[:, 0]
    vv = np.array(segments)[:, 2]
    return np.sum(ii * vv)


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

    print("part1", checksum(ss))


def part2(data):
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
                gg.append((pos, ch))
                pos += 1

    if DEBUG:
        print(displaystr(ss, gg))
    sidx = len(ss) - 1
    while sidx > 0:
        s = ss[sidx]  # segment end
        s_width = s[1]
        s0 = ss[sidx - s_width + 1]  # segment start
        sidx -= s_width
        hidx = 0
        while hidx < len(gg):
            h = gg[hidx]
            print(" " * h[0] + chalk.green("^") * h[1])
            hidx += h[1]
            # if the hole isn't big enough for segment,
            # try next
            if h[1] < s_width:
                continue

            # if it is big enough, set the hole data to be equal to the segment
            # then set the remaining size of the hole,
            # then sero out the segment's initial data
        if DEBUG:
            print(" " * s0[0] + chalk.red("V") * s_width)
            print(displaystr(ss, gg))
    #     hole_idx += 1
    #     segment_idx -= 1

    print("part2", checksum(ss))


DEBUG = True
if len(sys.argv) > 1:
    data = open(sys.argv[1]).read().strip()
else:
    data = sys.stdin.read().strip()
# part1(data)
part2(data)
