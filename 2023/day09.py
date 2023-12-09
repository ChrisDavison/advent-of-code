from utility import *

SAMPLE = """0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"""

DATA = Path("input/09").read_text()


@timed
def part1(data=SAMPLE):
    data = parser(data)
    s = 0
    for line in data:
        final = line[-1]
        while any(l != 0 for l in line):
            line = [y-x for x, y in zip(line, line[1:])]
            final += line[-1]
        s += final
    return s


@timed
def part2(data=SAMPLE):
    data = parser(data)
    s = 0
    for line in data:
        firsts = deque([line[0]])
        while any(l != 0 for l in line):
            line = [y-x for x, y in zip(line, line[1:])]
            firsts.insert(0, line[0])
        cur = 0
        for val in firsts:
            cur = val - cur
        # b-a...c-(b-a)...d -(c-(b-a))
        s += reduce(lambda a, b: b - a, firsts)
    return s



def parser(data):
    return parse(data, ints, lines, show=0)

part1(9)
part2(9)
