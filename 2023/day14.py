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

G = np.array(as_grid(DATA))
debug = False


def roll_cols(grid, up=True):
    # as grid is a numpy array, this modifies inplace
    for col in range(len(grid[0])):
        grid[:, col] = roll_slice(cat(grid[:, col]), to_right=not up)


def roll_rows(grid, right=False):
    # as grid is a numpy array, this modifies inplace
    for row in range(len(grid)):
        grid[row, :] = roll_slice(cat(grid[row, :]), to_right=right)


@cache
def roll_slice(l, to_right=True):
    if to_right:
        return roll_slice(l[::-1], to_right=False)[::-1]
    new = [c for c in l]
    for i in char_indices(new, "O"):
        # find previous #
        blocks_before = next(char_indices(new[:i], "#", reverse=True), 0)
        gapstart = 0
        if blocks_before:
            gapstart = blocks_before
        # find previous .
        gaps_before = [
            j for j, ch in enumerate(new) if ch == "." and j < i and j >= gapstart
        ]
        if gaps_before:
            new[i] = "."
            new[list(gaps_before)[0]] = "O"

    return new


def score_grid(grid):
    scores = np.arange(grid.shape[1], 0, -1)
    s = 0
    for i, row in enumerate(grid):
        s += sum(1 for c in row if c == "O") * scores[i]
    return s


timer()

g2 = np.copy(G)
roll_cols(g2, up=True)
score = score_grid(g2)
timer(f"Part 1: {score}")
# pyperclip.copy(int(score))

# ===== Part 2


def tumble(grid):
    actions = [
        (roll_cols, True),
        (roll_rows, False),
        (roll_cols, False),
        (roll_rows, True),
    ]
    for i, (f, b) in enumerate(actions):
        f(grid, b)


seen = dict()
timer(reset=True)
something = 0
remains = 0
left = 0

seen[un_grid(G)] = 1
cycles = 1000000000
loopstart = 0
for i in range(1, cycles):
    tumble(G)
    if un_grid(G) in seen:
        if not loopstart:
            loopstart = len(seen)
            seen = dict()
        else:
            loopend = i
            break
    else:
        seen[un_grid(G)] = 1

left = cycles - loopend
remain = left % len(seen)

# tumble it the remaining number of times, then score it
for i in range(left % len(seen)):
    tumble(G)

score = score_grid(G)
timer(f"Part 2: {score}")
pyperclip.copy(int(score))
