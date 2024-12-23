from utility import *
import numpy as np
import sys
from collections import Counter


if __name__ == "__main__":
    data = parse(sys.argv[1], atoms)

    lefts, rights = Xs(data), Ys(data)
    lefts = np.array(sorted(lefts))
    rights = np.array(sorted(rights))
    answer = np.sum(np.abs(lefts - rights))
    print(f"Part 1: {answer} (copied to clipboard)")

    s = 0
    right_counts = Counter(rights)
    answer = sum([left * right_counts[left] for left in lefts])
    print(f"Part 2: {answer}")
