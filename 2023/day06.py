import re
from utility import *

sample = """Time:      7  15   30
Distance:  9  40  200"""

source = sample
source = open("input/day06").read()

data = mapl(np.array, parse(6, ints, lines, show=0))

def f(tnum, dnum):
    s = 0
    for t in range(tnum):
        val = -(t * t) + tnum * t - dnum
        if val > 0:
            s += 1
    return s

def concat_ints(ls):
    return int(cat(mapl(str, ls)))

prod = 1
for time, record in zip(*data):
    prod *= f(int(time), int(record))
print(prod)

tnum = concat_ints(data[0])
dnum = concat_ints(data[1])

print(f(tnum, dnum))
