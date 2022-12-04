ex = """2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"""

def is_overlap(l, r):
    l = [int(p) for p in l.split('-')]
    r = [int(p) for p in r.split('-')]
    if l[0] <= r[0] and l[1] >= r[1]:
        return True
    if r[0] <= l[0] and r[1] >= l[1]:
        return True
    return False


def part1(data):
    overlaps = 0
    for row in data.splitlines():
        # print(row)
        l, r = row.split(',')
        overlaps += 1 if is_overlap(l, r) else 0
    return overlaps


def part2(data):
    overlaps = 0
    for row in data.splitlines():
        # print(row)
        l, r = row.split(',')
        l = [int(p) for p in l.split('-')]
        r = [int(p) for p in r.split('-')]
        lrange = set(range(l[0], l[1]+1))
        rrange = set(range(r[0], r[1]+1))
        has_overlap = lrange & rrange
        overlaps += 1 if has_overlap else 0
    return overlaps


print(part1(ex))
print(part1(open('inputs/04').read()))

print(part2(ex))
print(part2(open('inputs/04').read()))
