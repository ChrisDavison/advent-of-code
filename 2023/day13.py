from utility import *
from typing import *
from itertools import product
import pyperclip

timer()

SAMPLE = """#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"""

DATA = Path("input/13").read_text()

D = mapl(lines, paragraphs(DATA))
# D = mapl(lines, paragraphs(SAMPLE))


@cache
def is_vmirror(line, i):
    n = min(i, len(line) - i)
    return line[i - n : i][::-1] == line[i : i + n]


def column_mirrors(paragraph):
    def f(line):
        # start from 1 as we need at least 1 line to mirror from
        return {i for i in range(1, len(line)) if is_vmirror(line, i)}

    return reduce(lambda x, y: x & y, map(f, paragraph))


def is_row_mirrored(p, i):
    if i == 1:
        return True
    for n in range(1, len(p)):
        i1 = i - n - 1
        i2 = i + n
        if i1 < 0 or i2 >= len(p):
            return True
        if p[i1] != p[i2]:
            return False
    return True


def row_mirrors(paragraph):
    # walk until we find a matching row
    # then walk OUTWARD from that point
    # if any of the outward don't match
    mirrors = set()
    for i in range(1, len(paragraph)):
        u, d = paragraph[i - 1], paragraph[i]
        if u != d:
            continue
        if is_row_mirrored(paragraph, i):
            mirrors.add(i)
    return mirrors


def variants(paragraph):
    was = "."
    m = {"#": ".", ".": "#"}
    paragraph = paragraph[:]
    for x, y in product(range(len(paragraph)), range(len(paragraph[0]))):
        was = paragraph[x][y]
        paragraph[x][y] = m[was]
        yield ["".join(line) for line in paragraph]
        paragraph[x][y] = m[paragraph[x][y]]


s: int = 0
for paragraph in D:
    if h := column_mirrors(paragraph):
        s += list(h)[0]
    if v := row_mirrors(paragraph):
        s += 100 * list(v)[0]
timer(f"Part 1: {s}")
# clip(s)


s: int = 0
for paragraph in D:
    was_hmirror = column_mirrors(paragraph)
    was_vmirror = row_mirrors(paragraph)
    for variant in variants([[ch for ch in line] for line in paragraph]):
        cdelta = column_mirrors(variant) - was_hmirror
        rdelta = row_mirrors(variant) - was_vmirror
        if cdelta or rdelta:
            for mirror in cdelta:
                s += mirror

            for mirror in rdelta:
                s += mirror * 100
            break
timer(f"Part 2: {s}")
# clip(s)
