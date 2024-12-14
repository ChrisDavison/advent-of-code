import utility as u
import numpy as np
import sys
from pathlib import Path
from collections import defaultdict
from argparse import ArgumentParser
from dataclasses import dataclass


def quadrant(position, limits):
    midy, midx = np.floor(limits / 2).astype(int).tolist()
    x, y = position
    if x == midx or y == midy:
        return -1
    if x < midx:
        if y < midy:
            return 0
        return 1
    elif x > midx:
        if y < midy:
            return 2
        return 3
    return -1


@dataclass(slots=True, kw_only=True)
class Robot:
    xy: np.array
    delta: np.array

    def move(self, limits, steps=1):
        delta = self.delta * steps
        self.xy = (self.xy + delta) % limits


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("-d", "--debug", action="store_true", default=False)
    parser.add_argument("file", type=Path, nargs=1)
    args = parser.parse_args()

    DAYNUM = u.ints(Path(__file__).stem)[0]
    robots = []
    for line in args.file[0].read_text().strip().splitlines():
        nums = u.ints(line)
        robots.append(
            Robot(xy=np.array([nums[1], nums[0]]), delta=np.array([nums[3], nums[2]]))
        )

    # ---------------------------------------- Solution

    def display(grid):
        print(
            "\n".join(
                "".join(str(int(thing)) if thing >= 0 else "." for thing in row)
                for row in grid
            )
        )

    steps = 100
    limits = np.array([103, 101])
    positions = defaultdict(int)
    g = np.zeros(limits) - 1
    quadrants = np.zeros(4)
    midx, midy = np.floor(limits / 2).astype(int).tolist()
    for r in robots:
        r.move(limits, steps)
        positions[tuple(r.xy)] += 1

    for col in range(limits[1]):
        for row in range(limits[0]):
            g[row, col] = quadrant((row, col), limits[::-1])

    for r in sorted(robots, key=lambda x: tuple(x.xy)):
        quad = quadrant(r.xy, limits[::-1])
        if quad >= 0:
            quadrants[quad] += 1

    print(f"{np.prod(quadrants).astype(int)}")
