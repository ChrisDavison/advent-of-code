import utility as u
from pathlib import Path
from collections import deque
from simple_chalk import chalk
import functools as ft


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


def move(start, grid, direction):
    until_wall = []
    nx = u.Point2D(start.x, start.y)
    # print(f"{start=} + {direction} {direction.data}")
    rows, cols = len(grid), len(grid[0])
    while True:
        nx = nx + direction
        if nx.x < 0 or nx.x >= rows or nx.y < 0 or nx.y >= cols:
            print("out of bounds")
            break
        if grid[nx.x][nx.y] == WALL:
            break
        until_wall.append((nx, grid[nx.x][nx.y]))
        if grid[nx.x][nx.y] == SPACE:
            break
    # print(f"{until_wall=}")
    if any([ch == SPACE for pos, ch in until_wall]):
        # print(f"have a space in until_wall: {until_wall}")
        if until_wall[0][1] == SPACE:
            # the thing beside us is a gap, so just walk
            start = until_wall[0][0]
        else:
            pos_gap = [i for i, thing in enumerate(until_wall) if thing[1] == SPACE][0]
            gap_coord = until_wall[pos_gap][0]
            # print(f"{gap_coord=}")
            first_box_coord = until_wall[0][0]
            # print(f"{first_box_coord=}")
            grid[gap_coord.x][gap_coord.y] = "O"
            grid[first_box_coord.x][first_box_coord.y] = SPACE
            start = first_box_coord
    return start, grid


@ft.cache
def can_shove(point, grid, direction, step=0):
    spacing = step * 2 * " "
    next_point = point + direction
    next_sym = grid[next_point]
    nextpoint = grid[next_point] if next_point in grid else None
    print(
        f"{spacing}shove r{point.imag:.0f} c{point.real:.0f}? from {grid[point]} to {nextpoint}",
    )
    if next_sym == WALL:
        print(spacing + "FALSE")
        return False
    if grid[next_point] == SPACE:
        print(spacing + "TRUE")
        return True

    if grid[next_point] == BOXL:
        left = next_point
        right = next_point + 1

    else:
        left = next_point - 1
        right = next_point

    print(
        f"{spacing}...[] pair -> row {left.imag:.0f}, {left.real:.0f}..{right.real:.0f}"
    )
    left_ok = can_shove(left, grid, direction, step + 1)
    right_ok = can_shove(right, grid, direction, step + 1)
    print(spacing, (left_ok and right_ok))
    return left_ok and right_ok


def move_vertical(start, grid, direction):
    global DEBUG
    # double-wide
    u.arrow_directions = {"^": -1j, ">": 1, "v": 1j, "<": -1}
    above = deque()
    lhs, rhs = start, start
    found_wall = False

    # print("=" * 20 + " VERTICAL")
    # print(f"{start}")
    above = []
    # print()
    if not can_shove(start, grid, u.arrow_directions[direction]):
        return start, grid
    print("!!!!! SHOVE !!!!!")
    while not found_wall:
        lhs += u.arrow_directions[direction]
        rhs += u.arrow_directions[direction]
        if grid[lhs] == WALL or grid[rhs] == WALL:
            break
        if grid[lhs] == BOXR:
            lhs -= 1
        coldiff = int(rhs.real - lhs.real)
        # print(f"{lhs=} {rhs=} {coldiff=}")
        row = []
        for i in range(coldiff):
            point1 = lhs + i
            point2 = rhs + i
            if grid[point1] == WALL or grid[point2] == WALL:
                found_wall = True
                break
            if i == 0 and grid[point1] == BOXR:
                lhs -= 1
                row.append((point1 - 1, BOXL))
            if i == coldiff - 1 and grid[point2] == BOXL:
                rhs += 1
                row.append((point2 + 1, BOXR))
            row.append((point1, grid[point1]))
            row.append((point2, grid[point2]))
        # print(f"{row}")
        above.append(row)
        if all([ch == SPACE for p, ch in row]):
            break
    if not above:
        return start, grid
    if above == [[]]:
        step = start + u.arrow_directions[direction]
        if grid[step] == SPACE:
            start = step
    elif any([ch == SPACE for p, ch in above[-1]]):
        above = above[::-1]
        for row in above[:-1]:
            row = sorted(row, key=lambda x: x[0].real)
            for p, ch in row:
                direc = u.arrow_directions[direction]
                nextp = p - direc
                if grid[p] == SPACE and nextp in grid:
                    if grid[nextp] == BOXL:
                        if can_shove(p, grid, direc) and can_shove(p + 1, grid, direc):
                            grid[p] = grid[p - direc]
                            grid[p - direc] = SPACE
                    elif grid[nextp] == BOXR:
                        if can_shove(p, grid, direc) and can_shove(p - 1, grid, direc):
                            grid[p] = grid[p - direc]
                            grid[p - direc] = SPACE
                    else:  # '.'
                        grid[p] = grid[p - direc]
                        grid[p - direc] = SPACE
        start += u.arrow_directions[direction]
    return start, grid


def move_horizontal(start, grid, direction):
    until_wall = deque()
    nx = start
    while True:
        nx = nx + u.Point2D(*u.arrow_directions[direction])
        if grid[nx.y][nx.x] == WALL:
            break
        until_wall.append((nx, grid[nx]))
        if grid[nx.y][nx.x] == SPACE:
            break

    if any([ch == SPACE for pos, ch in until_wall]):
        # print(f"{until_wall=}")
        until_wall_s = deque([s for p, s in until_wall])
        until_wall_p = deque([p for p, s in until_wall])
        until_wall_s.rotate()
        # print(f"{until_wall_p}")
        # print(f"{until_wall_s}")
        for p, s in zip(until_wall_p, until_wall_s):
            grid[p] = s
        start = until_wall_p[0]
    return start, grid


def move2(start, grid, direction):
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
    if direction in "^v":
        start, grid = move_vertical(start, grid, direction)
    else:
        start, grid = move_horizontal(start, grid, direction)
    return start, grid


def display(grid, highlights=None):
    out = []
    for i, row in enumerate(grid):
        rowout = []
        for j, sym in enumerate(row):
            if sym == "#":
                rowout.append(chalk.blue(sym))
            elif sym in [BOXL, BOXR, BOX]:
                rowout.append(chalk.magenta(sym))
            else:
                rowout.append(sym)
        out.append(rowout)
    for h in highlights:
        if isinstance(h, list) or isinstance(h, tuple):
            h, c = h
            if isinstance(c, u.Point2D):
                c = chalk.red(c.data)
            else:
                c = chalk.red(c)
        else:
            c = chalk.red(out[h.x][h.y])
        out[h.x][h.y] = c
    print("\n".join("".join(row) for row in out))


def gps(x, y):
    # print(f"{coord.real} {coord.imag}")
    return 100 * y + x


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


def run(grid, start, rules, part2=False):
    for rule in rules:
        # print(rule)
        # display(grid, [(start, rule)])
        # print()
        if part2:
            start, grid = move2(start, grid, rule)
        else:
            start, grid = move(start, grid, rule)
        # input()
    return grid


def score_grid(grid):
    tot = 0
    for i, row in enumerate(grid):
        for j, ch in enumerate(row):
            if ch == "O":
                tot += gps(i, j)
    return int(tot)


def part1(filename):
    grid, start, rules = parse(filename)
    grid = run(grid, start, rules)
    print(score_grid(grid))


def part2(filename):
    grid, start, rules = parse(filename, doublewide=True)
    grid = run(grid, start, rules, part2=True)
    print(score_grid(grid))


DAYNUM = u.ints(Path(__file__).stem)[0]

part1(f"input/{DAYNUM}s")
part1(f"input/{DAYNUM}s2")
part1(f"input/{DAYNUM}")

# part2(f"input/{DAYNUM}s")
# part2(f"input/{DAYNUM}")
