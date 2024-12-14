from utility import ints
import numpy as np
import pyperclip
from collections import Counter
from argparse import ArgumentParser
from pathlib import Path


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("-d", "--debug", action="store_true", default=False)
    parser.add_argument("file", type=Path, nargs=1)
    args = parser.parse_args()

    lefts, rights = [], []
    for line in args.file[0].read_text().splitlines():
        d = ints(line)
        lefts.append(d[0])
        rights.append(d[1])
    lefts = np.array(sorted(lefts))
    rights = np.array(sorted(rights))
    answer = np.sum(np.abs(lefts - rights))
    print(f"Part 1: {answer} (copied to clipboard)")
    pyperclip.copy(answer)

    s = 0
    right_counts = Counter(rights)
    answer = sum([left * right_counts[left] for left in lefts])
    print(f"Part 2: {answer} (Hit enter to copy to clipboard)")
    input()
    pyperclip.copy(answer)
