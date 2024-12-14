import utility as u
import numpy as np
from pathlib import Path
from collections import defaultdict
from argparse import ArgumentParser
from dataclasses import dataclass


@dataclass(slots=True, kw_only=True)
class Robot:
    xy: np.array
    delta: np.array

    def move(self, limits, steps=1):
        delta = self.delta * steps
        self.xy = (self.xy + delta) % limits

    def quadrant(self, limits):
        midy, midx = np.floor(limits / 2).astype(int).tolist()
        x, y = self.xy
        if x == midx or y == midy:
            return -1
        return (2 if x > midx else 0) + (1 if y > midy else 0)


def display(grid):
    print(
        "\n".join(
            "".join(str(int(thing)) if thing >= 0 else "." for thing in row)
            for row in grid
        )
    )


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

    # part1
    steps = 100
    limits = np.array([103, 101])
    positions = defaultdict(int)
    g = np.zeros(limits) - 1
    quadrants = np.zeros(4)
    midx, midy = np.floor(limits / 2).astype(int).tolist()
    for r in robots:
        r.move(limits, steps)
        positions[tuple(r.xy)] += 1

    for r in sorted(robots, key=lambda x: tuple(x.xy)):
        quad = r.quadrant(limits[::-1])
        if quad >= 0:
            quadrants[quad] += 1

    print(f"part1 {np.prod(quadrants).astype(int)}")

    # part2
    print()
    entx = []
    enty = []
    for i in range(1, 10000):
        g = np.zeros(limits) - 1
        xs, ys = [], []
        for r in robots:
            r.move(limits, 1)
            g[r.xy[0], r.xy[1]] = "1"
            xs.append(r.xy[0])
            ys.append(r.xy[1])
        ex = int(np.var(xs))
        ey = int(np.var(ys))
        entx.append(ex)
        enty.append(ey)

        threshold = 0.8

        if ex < (threshold * np.mean(entx)) and ey < (threshold * np.mean(enty)):
            # add 'steps' to the i-count so we don't need to reset the robots
            print(i + steps, ex, ey)
            display(g)
            print()
