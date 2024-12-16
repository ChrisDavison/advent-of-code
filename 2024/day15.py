import os
import time
from pathlib import Path

from simple_chalk import chalk

import utility as u

North = u.Point2D(-1, 0, data="^")
South = u.Point2D(1, 0, data="v")
East = u.Point2D(0, 1, data=">")
West = u.Point2D(0, -1, data="<")
dirmap = {"^": North, ">": East, "v": South, "<": West}

WALL = "#"
BOX = "O"
BOXL = "["
BOXR = "]"
SPACE = "."


class WarehouseBot:
    def __init__(self, grid, start, rules, v2=False):
        self.grid = grid
        self.start = start
        self.rules = rules
        self.ruleidx = 0
        self.oldgrid = grid[:]
        self.oldstart = start
        self.highlights = [(start, rules[0].data)]
        self.v2 = v2
        self.show_old = False
        self.rows = len(grid)
        self.cols = len(grid[0])

    def fwd(self):
        self.oldgrid = self.grid[:]
        self.oldstart = self.start

        if self.ruleidx == len(self.rules):
            raise StopIteration
        if self.v2:
            self.move_v2()
        else:
            self.move()
        self.highlight([(self.start, self.rules[self.ruleidx].data)])
        self.ruleidx += 1

    def __str__(self, highlights=None):
        return self.__display_str(old=False)

    def __display_str(self, old=False):
        out = []
        source = self.grid
        if old:
            source = self.oldgrid
        for i, row in enumerate(source):
            rowout = []
            for j, sym in enumerate(row):
                if sym == "#":
                    rowout.append(chalk.blue(sym))
                elif sym in [BOXL, BOXR, BOX]:
                    rowout.append(chalk.yellow(sym))
                else:
                    rowout.append(chalk.gray(sym))
            out.append(rowout)
        for h in self.highlights:
            if isinstance(h, list) or isinstance(h, tuple):
                h, c = h
                if isinstance(c, u.Point2D):
                    c = chalk.red(c.data)
                else:
                    c = chalk.red(c)
            else:
                c = chalk.red(out[h.x][h.y])
            out[h.x][h.y] = c
        return "\n".join("".join(row) for row in out)

    def show_before_move(self):
        self.show_old = True
        return self

    def show_old_and_new(self):
        lhs = self.__display_str(old=True)
        rhs = self.__display_str(old=False)
        return "\n".join(
            lhs + "  " + rhs for lhs, rhs in zip(lhs.splitlines(), rhs.splitlines())
        )

    def highlight(self, highlights):
        self.highlights = highlights

    def clear_highlights(self):
        self.highlights = []

    def __repr__(self):
        return self.__str__()

    def run(self, show=False, pause=0):
        print("STARTING")
        print(self)
        print("-" * 40)
        n_rules = len(self.rules)
        for i, rule in enumerate(self.rules):
            self.fwd()
            if show and i > -1:
                print(f"{i+1} of {n_rules} {self.rules[i].data}")
                print(self)
                time.sleep(pause)
                input()
                # os.system("clear")
        # print("AFTER")
        # print(self)
        print(f"Score: {self.score()}")

    def move(self):
        until_wall = []
        direction = self.rules[self.ruleidx]
        nx = u.Point2D(self.start.x, self.start.y)
        # print(f"{self.start=} + {direction} {direction.data}")
        rows, cols = len(self.grid), len(self.grid[0])
        while True:
            nx = nx + direction
            if nx.x < 0 or nx.x >= rows or nx.y < 0 or nx.y >= cols:
                print("out of bounds")
                break
            if self.grid[nx.x][nx.y] == WALL:
                break
            until_wall.append((nx, self.grid[nx.x][nx.y]))
            if self.grid[nx.x][nx.y] == SPACE:
                break
        # print(f"{until_wall=}")
        if any([ch == SPACE for pos, ch in until_wall]):
            # print(f"have a space in until_wall: {until_wall}")
            if until_wall[0][1] == SPACE:
                # the thing beside us is a gap, so just walk
                self.start = until_wall[0][0]
            else:
                pos_gap = [
                    i for i, thing in enumerate(until_wall) if thing[1] == SPACE
                ][0]
                gap_coord = until_wall[pos_gap][0]
                # print(f"{gap_coord=}")
                first_box_coord = until_wall[0][0]
                # print(f"{first_box_coord=}")
                self.grid[gap_coord.x][gap_coord.y] = "O"
                self.grid[first_box_coord.x][first_box_coord.y] = SPACE
                self.start = first_box_coord

    def shove_horizontal(self, point, direction):
        ch = self.grid[point.x][point.y]
        if ch == "#":
            # If we reach a wall, we can't shove
            return False
        elif ch == ".":
            # If we reach a space, something can be shoved into this space
            return True
        elif ch == "O":
            # If we're in part1 and hit a box, only shove if the thing AFTER it can be shoved
            after = point + direction
            ax, ay = after.x, after.y
            can = self.shove_horizontal(after, direction)
            if can:
                self.grid[ax][ay] = ch
                return True
            else:
                return False
        else:
            if self.shove_horizontal(point + direction, direction):
                after = point + direction
                ax, ay = after.x, after.y
                self.grid[ax][ay] = ch
                return True
            else:
                return False

    def can_shove_vertical(self, points, direction):
        next_points = set()
        spaces = 0
        print(f"test shoving {points}")
        for point in points:
            ch = self.grid[point.x][point.y]
            np = point + direction
            # If anything in this layer is
            if ch == "#" or self.grid[np.x][np.y] == "#":
                # If we reach a wall, we can't shove
                return False
            elif ch == ".":
                # If we reach a space, something can be shoved into this space
                spaces += 1
            elif ch == "O":
                # If we're in part1 and hit a box, only shove if the thing AFTER it can be shoved
                # ax, ay = after.x, after.y
                next_points.add(point + direction)
                # can = self.shove_vertical(after, direction)
                # if can:
                #     self.grid[ax][ay] = ch
                #     self.grid[point.x][point.y] = "."
                #     return True
                # else:
                #     return False
            else:
                if ch == "]":
                    left = point + West
                    right = point
                elif ch == "[":
                    left = point
                    right = point + East
                else:
                    raise Exception(f"Unknown character when shoving {ch}")
                # ln = left + direction
                # rn = right + direction
                next_points.add(left + direction)
                next_points.add(right + direction)
                # if self.grid[ln.x][ln.y] == WALL or self.grid[rn.x][rn.y] == "WALL":
                #     return False
                # canl = self.shove_vertical(left + direction, direction, dryrun=True)
                # if not canl:
                #     return False
                # canr = self.shove_vertical(right + direction, direction, dryrun=True)
                # if not canr:
                #     return False
                # canl = self.shove_vertical(left + direction, direction, dryrun=False)
                # canr = self.shove_vertical(right + direction, direction, dryrun=False)
                # if not dryrun:
                #     left_after = left + direction
                #     right_after = right + direction
                #     self.grid[left_after.x][left_after.y] = "["
                #     self.grid[right_after.x][right_after.y] = "]"

                #     self.grid[left.x][left.y] = "."
                #     self.grid[right.x][right.y] = "."
                # return True

    def shove_vertical(self, points, direction):
        if all(self.shove_vertical(points, direction)):
            for p in points:
                np = p + direction
                self.grid[np.x][np.y] = self.grid[p.x][p.y]
                self.grid[p.x][p.y] = "."

    def move_v2(self):
        """This time, grid can be 2x wide (but not 2x tall)

        For shoving <>, now need to actually move grid as i'm not doing a single symbol-switch.
        For a top/bottom push, now need to potentially do MANY columns of manipulation...

                #[]    <- ignore
                []     <- ignore
                ..     <- shove to here
            []      <- shove
            [][]       <- shove
            []        <- shove
            @

        For every direction upwards, need to check (until the nearest wall) if I'm hitting a box
        If I hit the box, then need to fan-out from that area. Every tim ei touch a box, need to check
        the two columns above that...if either of them, also need to check all of their columns (only
        till the minimum of the nearest wall.
        """
        direction = self.rules[self.ruleidx]
        if direction.data in "^v":
            shoved = self.shove_vertical([self.start + direction], direction)
        else:
            shoved = self.shove_horizontal(self.start + direction, direction)

        if shoved:
            self.start = self.start + direction
            self.grid[self.start.x][self.start.y] = "."
        return self.start, self.grid

    def score(self):
        tot = 0
        for i, row in enumerate(self.grid):
            for j, ch in enumerate(row):
                if ch in [BOX, BOXL]:
                    tot += 100 * i + j
        return int(tot)


def parse(filename, doublewide=False):
    data = Path(filename).read_text()
    map, rules = u.paragraphs(data)
    rules = [
        dirmap[thing]
        for thing in "".join([line.strip() for line in rules.splitlines()])
    ]
    lines = map.splitlines()
    rows = len(lines)
    cols = len(lines[0])
    step = 2 if doublewide else 1
    g = [["." for _ in range(cols * step)] for _ in range(rows)]
    for rownum, row in enumerate(map.splitlines()):
        colnum = 0
        for ch in row:
            if ch == "@":
                start = u.Point2D(rownum, colnum)
                # also include starting position in floor tiles
                g[rownum][colnum] = SPACE
                if doublewide:
                    g[rownum][colnum + 1] = SPACE
            elif ch == WALL:
                g[rownum][colnum] = WALL
                if doublewide:
                    g[rownum][colnum + 1] = WALL
            elif ch == SPACE:
                g[rownum][colnum] = SPACE
                if doublewide:
                    g[rownum][colnum + 1] = SPACE
            else:
                g[rownum][colnum] = BOXL if doublewide else BOX
                if doublewide:
                    g[rownum][colnum + 1] = BOXR
            colnum += step
    print(start)
    return g, start, rules


def part1(filename):
    grid, start, rules = parse(filename)
    bot = WarehouseBot(grid, start, rules)
    bot.run(show=True, pause=0.1)


def part2(filename):
    grid, start, rules = parse(filename, doublewide=True)
    bot = WarehouseBot(grid, start, rules, v2=True)
    bot.run(show=True)


if __name__ == "__main__":
    DAYNUM = u.ints(Path(__file__).stem)[0]

    # part1(f"input/{DAYNUM}s")
    # part1(f"input/{DAYNUM}s2")
    # part1(f"input/{DAYNUM}")

    part2(f"input/{DAYNUM}s2")
    # part2(f"input/{DAYNUM}s")
    # part2(f"input/{DAYNUM}")
