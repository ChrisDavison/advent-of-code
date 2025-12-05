from utility import *
from collections import deque

def parser(inp):
    ds = [[positive_ints(line) for line in chunk.splitlines()] for chunk in open(inp).read().split("\n\n")]
    ranges = ds[0]
    vals = [x[0] for x in ds[1]]
    return ranges, vals


def simplify_ranges(ranges):
    ranges = sorted(ranges)
    # print(ranges)
    candidates = deque(ranges[1:])
    simplified = [ranges[0]]
    cs, ce = ranges[0]
    while candidates:
        rs, re = candidates.popleft()
        # print(f"{cs=} {ce=} {rs=} {re=}")
        if rs >= cs and rs <= ce:
            # range starts within current range
            simplified[-1] = [cs, max(ce, re)]
        else:
            simplified.append([rs, re])
            cs, ce = rs, re
    # print(simplified)
    return simplified

def part1(inp):
    ranges, vals = parser(inp)
    final = simplify_ranges(ranges)
    # print(f"{len(ranges)=}, {len(final)=}, {len(vals)=}")
    tot = 0
    for val in vals:
        for rs, re in final:
            if val >= rs and val <= re:
                tot += 1
                break
    print(tot)
    # print(sum(1 for v in vals if v in final))

def part2(data):
    pass

part1("input/05s")
part1("input/05")

simplify_ranges([(3, 10), (8, 15), (18, 20)])
