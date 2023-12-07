import re
from utility import *

sample = """Time:      7  15   30
Distance:  9  40  200"""

source = sample
source = Path("input/06").read_text()

data = mapl(np.array, parse(6, ints, lines, show=0))

def f(t, d):
    # quadratic equation
    # |
    # |     ________
    # |    /        \
    # |---x----------x------------
    # |  /            \
    # | /              \
    # |/                \
    # |===========================
    # want to find the two roots x that give us values above the record
    # take ceil of first, so we're after we cross above
    # take floor of second, so we're before the cross below
    first = m.ceil((-t + m.sqrt(t*t - 4*d)) / -2)
    second = m.floor((-t - m.sqrt(t*t - 4*d)) / -2)
    return np.abs(first - second) + 1

def concat_ints(ls):
    return int(cat(mapl(str, ls)))

@timed
def part1():
    prod = 1
    for time, record in zip(*data):
        prod *= f(int(time), int(record))
    print(prod)
part1()

@timed
def part2():
    tnum = concat_ints(data[0])
    dnum = concat_ints(data[1])

    print(f(tnum, dnum))
part2()
