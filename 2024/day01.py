from utility import ints
import numpy as np
import pyperclip
from pathlib import Path
from collections import Counter

TEST_INPUT = """3   4
4   3
2   5
1   3
3   9
3   3"""

DATA = Path("input/01").read_text()

l1, l2 = [], []
source = DATA
# source = TEST_INPUT
for line in source.splitlines():
    d = ints(line)
    l1.append(d[0])
    l2.append(d[1])
l1 = np.array(sorted(l1))
l2 = np.array(sorted(l2))
answer = np.sum(np.abs(l1 - l2))
print(f"Part 1: {answer} (copied to clipboard)")
pyperclip.copy(answer)

s = 0
l2_counts = Counter(l2)
answer = sum([l * l2_counts[l] for l in l1])
print(f"Part 2: {answer} (Hit enter to copy to clipboard)")
input()
pyperclip.copy(answer)
