from utility import *
import os

SAMPLE = """O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."""

DATA = Path("input/14").read_text()

use = DATA

dim = get_dim(use)
rolled = np.zeros(dim, dtype=int)
current_column_fill = np.zeros(dim[1], dtype=int) - 1

# print(rolled)
# print(current_column_fill)
# print('-'*50)

def roll(grid, direc='n'):
    pass
   
timer()
for y, line in enumerate(use.splitlines()):
    # print(line)
    # print(current_column_fill)
    # print()
    for x, c in enumerate(line):
        colheight = current_column_fill[x]
        # print(f"{colheight = :}")
        if c == '.':
            continue
        if c == '#':
            rolled[y][x] = 9
            current_column_fill[x] = y
        elif c == 'O':
            rolled[colheight+1][x] = 1
            current_column_fill[x] = colheight+1
    # print(rolled[:y])
    # print()
    # input()
    # os.system('clear')
    # break

# print(rolled)

scores = np.arange(dim[1], 0, -1)

s = 0
m = {0: ' ', 1: 'O', 9: '░'}
for i, row in enumerate(rolled):
    print(cat([m[c] for c in row]))
    s += sum(1 for c in row if c == 1) * scores[i]

timer(f"Part 1: {s}")
pyperclip.copy(int(s))
