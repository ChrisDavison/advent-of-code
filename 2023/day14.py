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

G = np.array(as_grid(SAMPLE))
debug = False


def prettify(grid):
    out = ""
    for line in grid:
        for ch in line:
            if ch == '.':
                out += ' '
            elif ch == '#':
                out += '░'
            else:
                out += '*'
        if len(grid) > 1:
            out += '\n'
    return out


def roll_cols(grid, up=True):
    for col in range(len(grid[0])):
        grid[:, col] = roll_slice(cat(grid[:, col]), to_right=not up)
    return grid

def roll_rows(grid, right=False):
    for row in range(len(grid)):
        grid[row,:] = roll_slice(cat(grid[row, :]), to_right=right)
    return grid

@cache
def roll_slice(l, to_right=True):
    if to_right:
        # if debug:
        #     print(">> right")
        return roll_slice(l[::-1], to_right=False)[::-1]
    # if debug:
    #     print("<< left")
    new = [c for c in l]
    if debug:
        print(cat(mapt(str, list(range(len(new))))))
        print(cat(new))
        print()
    for i in char_indices(new, 'O'):
        if debug:
            print(' ' * (i) + '↓')
            print(f"{cat(new)}")
        # find previous #
        blocks_before = next(char_indices(new[:i], '#', reverse=True), 0)
        if debug:
            print(f"# = {blocks_before}")
        gapstart = 0
        if blocks_before:
            gapstart = blocks_before
        # find previous .
        gaps_before = [j for j, ch in enumerate(new) if ch == '.' and j < i and j >= gapstart]
        
        if debug:
            print(f". = {gaps_before}")
        if gaps_before:
            new[i] = '.'
            new[list(gaps_before)[0]] = 'O'

        if debug:
            print(cat(new))
            print()

    return new


def score_grid(grid):
    scores = np.arange(G.shape[1], 0, -1)
    s = 0
    for i, row in enumerate(G):
        s += sum(1 for c in row if c == 'O') * scores[i]
    return s

timer()
# roll_cols(G, up=True)
# score = score_grid(G)
# timer(f"Part 1: {score}")
# pyperclip.copy(int(score))

# ===== Part 2

cycles = 1000000000
seen = dict()
timer(reset=True)
something = 0
remains = 0
left = 0
for i in range(1, cycles):
    if i % 10_000 == 0:
        print(i)
        timer()

    G = roll_cols(G, up=True)
    G = roll_rows(G, right=False)
    G = roll_cols(G, up=False)
    G = roll_rows(G, right=True)
    if un_grid(G) in seen:
        print(f"{seen = :}")
        something = i - len(seen)
        remain = (cycles - i) % something
        left = cycles - remains
        print(f"{something = :}")
        print(f"{remain = :}")
        print(f"{left = :}")
        break
    else:
        seen[un_grid(G)] = 1

print(f"{left = :}")
# rem = cycles - (n_cycles * len(seen))

# for i in range(left):
#     roll_cols(G, up=True)
#     roll_rows(G, right=False)
#     roll_cols(G, up=False)
#     roll_rows(G, right=True)

    # print(prettify(G))

    # score = score_grid(G)
    # print(f"{i} {score = :}")
# timer(f"Part 2: {score}")
# pyperclip.copy(int(score))

