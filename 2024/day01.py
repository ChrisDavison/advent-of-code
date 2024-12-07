from utility import ints
import sys
import numpy as np
import pyperclip
from collections import Counter


lefts, rights = [], []
for line in sys.stdin().readlines():
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
