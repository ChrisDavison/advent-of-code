import sys
import string
import os
from utility import Point2D
from collections import defaultdict
import itertools as it
from simple_chalk import chalk

year = 2024
day = 8
prefix = f"{year}.{day:02d}."


def within_lims(x, y, width, height):
    return x >= 0 and x <= width and y >= 0 and y <= height


DATA = open(sys.argv[1]).read()

letters = defaultdict(list)
width, height = 0, 0
for y, row in enumerate(DATA.splitlines()):
    height = max(y, height)
    for x, ch in enumerate(row):
        width = max(width, x)
        if ch in string.ascii_letters or ch in string.digits:
            letters[ch].append(Point2D(x, y))


grid2 = [[c for c in row] for row in DATA.splitlines()]


def calculate(antenna_positions, *, infinite=False, step=False):
    antinodes = set()
    done = set()
    for letter, positions in antenna_positions.items():
        antenna_pairs = [
            (p1, p2) for p1, p2 in it.product(positions, positions) if p1 != p2
        ]
        for p1, p2 in antenna_pairs:
            antinodes.add((p1.x, p1.y))
            antinodes.add((p2.x, p2.y))
            if p1.x > p2.x:
                p1, p2 = p2, p1
            key = p1.x, p1.y, p2.x, p2.y
            if key in done:
                continue
            dx, dy = p2.x - p1.x, p2.y - p1.y
            grid2[p1.y][p1.x] = chalk.red(letter)
            grid2[p2.y][p2.x] = chalk.red(letter)

            # Just start looping for harmonics
            a1, a2 = p1, p2
            while True:
                added1, added2 = False, False
                if step:
                    os.system("clear")
                a1 = a1.shift(-dx, -dy)
                if within_lims(a1.x, a1.y, width, height):
                    grid2[a1.y][a1.x] = chalk.bold.green("#")
                    antinodes.add((a1.x, a1.y))
                    added1 = True

                a2 = a2.shift(dx, dy)
                if within_lims(a2.x, a2.y, width, height):
                    grid2[a2.y][a2.x] = chalk.bold.green("#")
                    antinodes.add((a2.x, a2.y))
                    added2 = True

                if not added1 and not added2:
                    # both are out of bounds
                    break
                if step:
                    print("\n".join("".join(g) for g in grid2))
                    input()
                # But if we're not infinitely expanding, just break early
                if not infinite:
                    break

            grid2[p1.y][p1.x] = letter
            grid2[p2.y][p2.x] = letter
            if added1:
                grid2[a1.y][a1.x] = "#"
            if added2:
                grid2[a2.y][a2.x] = "#"
            done.add(key)
    print(len(antinodes))


calculate(letters)
calculate(letters, infinite=True)
