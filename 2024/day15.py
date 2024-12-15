import utility as u
from pathlib import Path
from collections import defaultdict
from simple_chalk import chalk

DAYNUM = u.ints(Path(__file__).stem)[0]
# data = Path(f"input/{DAYNUM}s").read_text()
# data = Path(f"input/15s2").read_text()
data = Path(f"input/{DAYNUM}").read_text()

map, rules = u.paragraphs(data)
rules = "".join([line.strip() for line in rules.splitlines()])
boxes = defaultdict()
walls = set()
start = None
for rownum, row in enumerate(map.splitlines()):
    for colnum, ch in enumerate(row):
        coord = colnum + 1j * rownum
        if ch == "@":
            start = coord
            # also include starting position in floor tiles
            boxes[coord] = "."
        elif ch == "#":
            walls.add(coord)
        else:
            boxes[coord] = ch


def move(start, walls, boxes, direction):
    dirs = {"^": -1j, ">": 1, "v": 1j, "<": -1}
    until_wall = []
    nx = start
    while True:
        nx += dirs[direction]
        if nx in walls:
            break
        until_wall.append((nx, boxes[nx]))
        if boxes[nx] == ".":
            break
    if any([ch == "." for pos, ch in until_wall]):
        if until_wall[0][1] == ".":
            # the thing beside us is a gap, so just walk
            start = until_wall[0][0]
        else:
            # we need to shove boxes
            # want    @ 0 0 .
            # become  . @ 0 0

            # because of the way I'm doing this, I'm only ever going to have
            # either a single gap, or a sequence of O into a gap (i use gap or wall as loop's break)
            # so I just need to set the gap's ch to box, and the first box's ch to '.'
            # then set start to the first box's position
            pos_gap = [i for i, thing in enumerate(until_wall) if thing[1] == "."][0]
            gap_coord = until_wall[pos_gap][0]
            first_box_coord = until_wall[0][0]
            boxes[gap_coord] = "O"
            boxes[first_box_coord] = "."
            start = first_box_coord
    return start, walls, boxes


def display(robot, walls, boxes, last_move):
    rows = int(max([pos.imag for pos in walls])) + 1
    cols = int(max([pos.real for pos in walls])) + 1
    grid = [[" " for _ in range(cols)] for _ in range(rows)]
    for pos in walls:
        row = int(pos.imag)
        col = int(pos.real)
        grid[row][col] = chalk.blue("#")
    for pos, ch in boxes.items():
        row = int(pos.imag)
        col = int(pos.real)
        if ch == ".":
            grid[row][col] = ch
        else:
            grid[row][col] = chalk.magenta(ch)
    grid[int(robot.imag)][int(robot.real)] = chalk.bold.red("@")
    print(f"MOVE {last_move}")
    print("\n".join("".join(row) for row in grid))


print("START")
display(start, walls, boxes, "")

for rule in rules:
    start, walls, boxes = move(start, walls, boxes, rule)
    # display(start, walls, boxes, rule)
    # input()


def gps(coord):
    # print(f"{coord.real} {coord.imag}")
    return 100 * coord.imag + coord.real


tot = 0
for b, ch in boxes.items():
    if ch == "O":
        tot += gps(b)

print(f"{tot}")
