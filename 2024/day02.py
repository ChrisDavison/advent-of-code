from utility import ints
from argparse import ArgumentParser
from pathlib import Path


def check(levels):
    samesign = all([x > 0 for x in d]) or all([x < 0 for x in d])
    if all([abs(x) in [1, 2, 3] for x in d]) and samesign:
        return True


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("-d", "--debug", action="store_true", default=False)
    parser.add_argument("file", type=Path, nargs=1)
    args = parser.parse_args()

    data = args.file[0].read_text().splitlines()

    P = []
    s = 0
    for line in data:
        i = ints(line)
        d = [b - a for a, b in zip(i, i[1:])]
        if check(d):
            s += 1
    print(f"Part 1: {s}")

    P = []
    s = 0
    for line in data:
        i = ints(line)
        for j in range(len(i)):
            subset = i[:j] + i[j + 1 :]
            d = [b - a for a, b in zip(subset, subset[1:])]
            if check(d):
                s += 1
                break
    print(f"Part 2: {s}")
