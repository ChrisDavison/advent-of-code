import sys
from dataclasses import dataclass
from collections import deque
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
        maxpos = max(maxpos, s.pos)
    for g in gg:
        maxpos = max(maxpos, g.pos)
    disp = np.zeros(maxpos + 1, dtype=int)
    for s in ss:
        disp[s.pos] = s.id
    for g in gg:
        disp[g.pos] = -1

    stuff = disp.astype(str).tolist()
    if last_change:
        a, b = last_change
        stuff[a.pos] = chalk.red("^")
        stuff[b] = chalk.red(stuff[b])
    output = "".join(stuff).replace("-1", ".")
    return output


def checksum(segments):
    ii = np.array([s.pos for s in segments])
    vv = np.array([s.id for s in segments])
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
                ss.append(Segment(pos, ch, s_idx))
                pos += 1
            s_idx += 1
        else:
            for j in range(ch):
                gg.append(Gap(pos, ch))
                pos += 1

    hole_idx = 0
    segment_idx = len(ss) - 1
    if DEBUG:
        print(displaystr(ss, gg))
    while True:
        if hole_idx >= len(gg) or segment_idx < 0:
            break
        if gg[hole_idx].pos > ss[segment_idx].pos:
            break
        gg[hole_idx], ss[segment_idx] = (
            Gap(ss[segment_idx].pos, gg[hole_idx].n),
            Segment(gg[hole_idx].pos, ss[segment_idx].n, ss[segment_idx].id),
        )
        if DEBUG:
            print(displaystr(ss, gg, last_change=(gg[hole_idx], ss[segment_idx].pos)))
        hole_idx += 1
        segment_idx -= 1

    print("part1", checksum(ss))


def part2(data):
    gg = []
    pos = 0
    s_idx = 0
    arr = []
    for i, ch in enumerate(data):
        ch = int(ch)
        if ch == 0:
            continue
        if i % 2 == 0:
            for j in range(ch):
                arr.append(s_idx)
                pos += 1
            s_idx += 1
        else:
            for j in range(ch):
                arr.append(-1)
                gg.append(Gap(pos, ch))
                pos += 1

    arr = np.array(arr, dtype=int)
    # print(arr)

    def arr2str(arr, colour_indices=None):
        vals = [str(s) if s >= 0 else "." for s in arr]
        if colour_indices is not None:
            vals = [
                chalk.red(s) if i in colour_indices else s for i, s in enumerate(vals)
            ]
        print("".join(vals))

    target_id = np.max(arr)

    gaps = np.where(arr < 0)[0]
    gap_blocks = [[int(gaps[0])]]
    for g in gaps[1:]:
        if (g - gap_blocks[-1][-1]) == 1:
            gap_blocks[-1].append(int(g))
        else:
            gap_blocks.append([int(g)])

    last_idx = len(arr)
    while target_id > 0:
        indices = np.where(arr[:last_idx] == target_id)[0]
        last_idx = indices[0]
        size = len(indices)
        first_gap = [(i, g) for i, g in enumerate(gap_blocks) if len(g) >= size]
        if first_gap:
            first_gap = first_gap[0]
            a, b = first_gap[1][0], first_gap[1][0] + size
            if a < indices[0]:
                arr[a:b] = target_id
                arr[indices] = -1
                # arr2str(arr, [*indices, *first_gap[1]])
                rem = len(first_gap[1]) - size
                if rem == 0:
                    gap_blocks.pop(first_gap[0])
                else:
                    gap_blocks[first_gap[0]] = gap_blocks[first_gap[0]][size:]
        target_id -= 1
    arr[np.where(arr < 0)[0]] = 0
    check = np.sum(arr * np.arange(len(arr)))
    print("part2", check)


DEBUG = False
if len(sys.argv) > 1:
    data = open(sys.argv[1]).read().strip()
else:
    data = sys.stdin.read().strip()
part1(data)
part2(data)
