from utility import *

def parser(inp):
    ds = [[positive_ints(line) for line in chunk.splitlines()] for chunk in open(inp).read().split("\n\n")]
    ranges = [list(d) for d in ds[0]]
    vals = [x[0] for x in ds[1]]
    return simplify_ranges(ranges), vals


def simplify_ranges(ranges):
    def intersect(a,b):
        if b[0]<a[0]:
            # b0...a0
            return intersect(b,a)
        # a0...b0...a1 ?
        return b[0]<=a[1]

    def union(a,b):
        if b[0]<a[0]:
            # b0...a0
            return union(b,a)
        # a0....max[a1, b1]
        return (a[0],max(a[1],b[1]))

    simplified = [ranges[0]]
    for original in ranges[1:]:
        # build a new 'simplified ranges' list
        new_rr = []
        for simpler in simplified:
            if intersect(simpler,original):
                # we can simplify by merging two ranges
                original = union(simpler,original)
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
        for (rs, re) in ranges:
            if rs <= val <= re:
                tot += 1
                break
    print(tot)

def part2(inp):
    ranges, _vals = parser(inp)
    tot = 0
    for rs, re in ranges:
        tot += re - rs + 1
    print(tot)


part1("input/05s")
part1("input/05")

part2("input/05s")
part2("input/05")

