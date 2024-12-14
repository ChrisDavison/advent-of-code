import utility as u
from pathlib import Path
from argparse import ArgumentParser
import numpy as np
import tqdm


def solve(a, b, prize, part2=False):
    px, py = prize
    if part2:
        px += 10000000000000
        py += 10000000000000
    ax, ay = a
    bx, by = b
    denom = ax * by - ay * bx
    A = (by * px - bx * py) / denom
    B = (ax * py - ay * px) / denom
    if A == int(A) and B == int(B):
        return int(3 * A + B)
    return 0


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("-d", "--debug", action="store_true", default=False)
    parser.add_argument("file", type=Path, nargs=1)
    args = parser.parse_args()

    DAYNUM = u.ints(Path(__file__).stem)[0]
    data = u.paragraphs(args.file[0].read_text().strip())

    parsed = [u.mapl(u.ints, chunk.splitlines()) for chunk in data]

    # ---------------------------------------- Solution
    min_tokens = 0
    for i, p in tqdm.tqdm(enumerate(parsed)):
        min_tokens += solve(*p)
    print(min_tokens)

    min_tokens2 = 0
    for i, p in enumerate(parsed):
        min_tokens2 += solve(*p, part2=True)
    print(min_tokens2)
