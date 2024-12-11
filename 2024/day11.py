from utility import ints
from pathlib import Path
import numpy as np
from math import log10
import sys
from functools import cache

if len(sys.argv) > 1:
    if Path(sys.argv[1]).is_file():
        DATA = Path(sys.argv[1]).read_text()
else:
    DATA = input("Vals: ")

data = np.array(ints(DATA), dtype=int)

print(data)


@cache
def n_digits(d):
    return int(log10(d) + 1)


@cache
def split_stone(d):
    s = str(d)
    n = int(n_digits(d) / 2)
    l, r = s[:n], s[n:]
    return int(l), int(r)


pos = 0

for i in range(75):
    data2 = np.zeros(len(data) * 2 + 1, dtype=int) - 1
    pos = 0
    for d in data[data >= 0]:
        if d == 0:
            data2[pos] = 1
        elif (n_digits(d) % 2) == 0:  # even number of digits
            l, r = split_stone(d)
            data2[pos] = l
            pos += 1
            data2[pos] = r
        else:
            data2[pos] = d * 2024
            pos += 1
        pos += 1
    data = data2[data2 >= 0]
    print(i, data.size)
    # print(f"{i+1:<10}{data}")
# print(data2)
# print(np.sum([d for d in data2 if d > 0]))
print(data.size)
