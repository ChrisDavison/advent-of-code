import utility as u
import os
from pathlib import Path
from collections import deque
from simple_chalk import chalk


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
        self.highlights = [(start, self.rules[0].data)]
        self.v2 = v2

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

    def undo(self):
        self.grid = self.oldgrid[:]
        self.start = self.oldstart
        self.ruleidx -= 1

    def __str__(self, highlights=None):
        out = []
        for i, row in enumerate(self.grid):
            rowout = []
            for j, sym in enumerate(row):
                if sym == "#":
                    rowout.append(chalk.blue(sym))
                elif sym in [BOXL, BOXR, BOX]:
                    rowout.append(chalk.magenta(sym))
                else:
                    rowout.append(sym)
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

    def highlight(self, highlights):
        self.highlights = highlights

    def clear_highlights(self):
        self.highlights = []

    def __repr__(self):
        return self.__str__()

    def __iter__(self):
        return self

    def __next__(self):
        self.fwd()
        return self.start, self.rules[self.ruleidx - 1]

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

    def can_shove(self, points, step=0):
        spacing = step * 2 * " "
        # print(spacing, points)
        direction = self.rules[self.ruleidx]
        next_pairs = set()
        for p in points:
            ch = self.grid[p.x][p.y]
            if ch == BOXL:
                next_pairs.add((p + direction, BOXL))
                next_pairs.add((p + direction + East, BOXR))
            elif ch == BOXR:
                next_pairs.add((p + direction + West, BOXL))
                next_pairs.add((p + direction, BOXR))
            else:
                next_pairs.add((p + direction, ch))
        if all(map(lambda x: x[1] == ".", next_pairs)):
            # print(spacing, "ALL SPACES")
            # print(self)
            # print("...to...")
            # print(f"{next_pairs}")
            # for p, ch in next_pairs:
            #     prev = p - direction
            #     print(f"shoving {prev} into {p}")
            #     self.grid[p.x][p.y] = self.grid[prev.x][prev.y]
            #     self.grid[prev.x][prev.y] = "."
            #     print(self)
            #     input()
            # self.grid[p.x][p.y] = "."
            return True

        if any(map(lambda x: x[1] == "#", next_pairs)):
            # print(spacing, "WALL")
            return False
        next_points = [p for p, ch in next_pairs if ch != "."]
        if self.can_shove(next_points, step + 1):
            # print(spacing, "some boxes")
            # print(self)
            # print("...to...")
            for p, ch in next_pairs:
                prev = p - direction
                # print(f"shoving {prev} into {p}")
                self.grid[p.x][p.y] = self.grid[prev.x][prev.y]
                self.grid[prev.x][prev.y] = "."
                # print(self)
                # input()
            return True
        return False

    def move_vertical(self):
        direction = self.rules[self.ruleidx]
        nextp = self.start + direction
        points = []
        if self.grid[nextp.x][nextp.y] == BOXL:
            points = [nextp, nextp + East]
        elif self.grid[nextp.x][nextp.y] == BOXR:
            points = [nextp + West, nextp]
        else:
            points = [nextp]

        if not self.can_shove(points):
            # print("NO SHOVE")
            return self.start, self.grid
        # print("!!!!! SHOVE !!!!!")
        self.start = self.start + direction
        return self.start, self.grid
        # raise Exception("Fix vertical shove logic")

    def move_horizontal(self):
        direction = self.rules[self.ruleidx]
        until_wall = deque()
        nx = self.start
        cols = len(self.grid[0])
        rows = len(self.grid)

        def in_bounds(point):
            return point.x >= 0 and point.x < rows and point.y >= 0 and point.y < cols

        # print(f"H{nx} {direction.data} {direction}")
        while True:
            nx = nx + u.Point2D(direction.x, direction.y)
            # print(f"{nx=} {self.grid[nx.x][nx.y]}")
            if not in_bounds(nx):
                break
            if self.grid[nx.x][nx.y] == WALL:
                # print(f"{WALL}")
                break
            until_wall.append((nx, self.grid[nx.x][nx.y]))
            if self.grid[nx.x][nx.y] == SPACE:
                break

        # print(f"{until_wall=}")
        if any([ch == SPACE for pos, ch in until_wall]):
            # print(f"{until_wall=}")
            until_wall_s = deque([s for p, s in until_wall])
            until_wall_p = deque([p for p, s in until_wall])
            until_wall_s.rotate()
            # print(f"{until_wall_p}")
            # print(f"{until_wall_s}")
            for p, s in zip(until_wall_p, until_wall_s):
                # print(f"{p, s}")
                self.grid[p.x][p.y] = s
            self.start = until_wall_p[0]
        return self.start, self.grid

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
        if self.rules[self.ruleidx].data in "^v":
            start, grid = self.move_vertical()
        else:
            start, grid = self.move_horizontal()
        return start, grid

    def score(self):
        tot = 0
        tot2 = 0
        for i, row in enumerate(self.grid):
            for j, ch in enumerate(row):
                if ch == "O":
                    tot += 100 * i + j
                if ch == "[":
                    tot2 += 100 * i + j
        return int(tot), int(tot2)


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
    # print(start)
    return g, start, rules


def part1(filename):
    grid, start, rules = parse(filename)
    bot = WarehouseBot(grid, start, rules)
    for move in bot:
        print(bot)
        # input()
    print(bot.score())


def part2(filename):
    grid, start, rules = parse(filename, doublewide=True)
    bot = WarehouseBot(grid, start, rules, v2=True)
    print(bot)
    for move in bot:
        print(bot)
        input()
        os.system("clear")
    print(bot.score())


if __name__ == "__main__":
    DAYNUM = u.ints(Path(__file__).stem)[0]

    # part1(f"input/{DAYNUM}s")
    # part1(f"input/{DAYNUM}s2")
    # part1(f"input/{DAYNUM}")

    # part2(f"input/{DAYNUM}s2")
    # part2(f"input/{DAYNUM}s")
    part2(f"input/{DAYNUM}")
