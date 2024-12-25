from utility import *
import sys
import numpy as np
import itertools as it

chunks = parse("25", section_by=paragraphs)
locks = []
keys = []
for i, chunk in enumerate(chunks):
    grid = np.array([[c for c in line] for line in chunk.splitlines()]).T
    print("-" * 40)
    # print(f"{i}\n{grid}")
    print()
    lock = []
    key = []
    is_lock = grid[0][0] == "#"
    for col in grid:
        c = "".join(mapt(str, col))
        if is_lock:
            indices = int(np.argwhere(col == "#")[-1]) + 1
            print(f"{c} {indices}")
            lock.append(indices)
        else:
            indices = int(np.argwhere(col == ".")[-1]) + 1
            print(f"{c} {indices}")
            key.append(len(c) - indices)
    if is_lock:
        locks.append(lock)
    else:
        keys.append(key)
    # break
valid = 0
for l, k in it.product(locks, keys):
    s = np.array(l) + np.array(k)
    if np.all(s <= 7):
        valid += 1
print(f"{locks}")
print(f"{keys}")
print(f"{valid=}")
# data = parse("25")
