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

timer()
D = mapl(lines, paragraphs(DATA))
# D = mapl(lines, paragraphs(SAMPLE))

def is_vmirror(line, i):
    l, r = line[:i], line[i:]
    n = min(len(l), len(r))
    return l[-n:][::-1] == r[:n]


def column_mirrors(paragraph):
    def f(line):
        possible = set()
        for i in range(1, len(line)):
            N = len(line)
            w = N - i
            if is_vmirror(line, i): 
                possible.add(i)
        return possible

    possible = f(paragraph[0])
    for i, line in enumerate(paragraph[1:]):
        if not possible:
            break
        possible &= f(line)
    return possible


def is_row_mirrored(p, i):
    if i == 1:
        return True
    found = True
    for n in range(1, len(p)):
        i1 = i - n - 1
        i2 = i + n
        if i1 < 0 or i2 >= len(p):
            return True
        u, d = p[i-n-1], p[i+n]
        spc = (n+1) * ' '
        if u != d:
            return False
    return True


def row_mirrors(paragraph):
    # walk until we find a matching row
    # then walk OUTWARD from that point
    # if any of the outward don't match
    mirrors = set()
    for i in range(1, len(paragraph)):
        u, d = paragraph[i-1], paragraph[i]
        if u != d:
            continue
        if is_row_mirrored(paragraph, i):
            mirrors.add(i)
    return mirrors
   
# row_mirrors(D[1])


def variants(paragraph):
    was = '.'
    m = {'#': '.', '.': '#'}
    paragraph = paragraph[:]
    for (x, y) in product(range(len(paragraph)), range(len(paragraph[0]))):
        was = paragraph[x][y]
        paragraph[x][y] = m[was]
        yield [''.join(line) for line in paragraph]
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
