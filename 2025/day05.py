from utility import *


def parser(inp):
    ds = [
        [positive_ints(line) for line in chunk.splitlines()]
        for chunk in open(inp).read().split("\n\n")
    ]
    ranges = [Interval(d[0], d[1]) for d in ds[0]]
    vals = [x[0] for x in ds[1]]
    return simplify_ranges(ranges), vals


def simplify_ranges(ranges):
    simplified = [ranges[0]]
    for original in ranges[1:]:
        # build a new 'simplified ranges' list
        new_rr = []
        for simpler in simplified:
            if simpler.intersect(original):
                # we can simplify by merging two ranges
                original = simpler.union(original)
            else:
                # otherwise, this range _might_ be standalone
                new_rr.append(simpler)
        new_rr.append(original)
        simplified = new_rr
    return simplified


def part1(inp):
    ranges, vals = parser(inp)
    tot = 0
    for val in vals:
        for r in ranges:
            if r.contains(val):
                tot += 1
                break
    print(tot)


def part2(inp):
    ranges, _vals = parser(inp)
    tot = 0
    for r in ranges:
        tot += (r.end - r.start) + 1
    print(tot)


part1("input/05s")
part1("input/05")

part2("input/05s")
part2("input/05")
