from utility import *

def part1(data):
    # part1
    vals = [(row[0], int(row[1:])) for row in data]
    start = 50
    part1 = 0
    for dir, val in vals:
        if dir == "R":
            start += val
        else:
            start -= val
        start %= 100
        if start == 0:
            part1 += 1
    print(f"{part1 = }")

def part2(data):
    vals = [(row[0], int(row[1:])) for row in data]
    start = 50
    part2 = 0
    for dir, val in vals:
        sign = 1
        if dir == 'L':
            sign = -1
        for _ in range(val):
            start += sign
            start %= 100
            if start == 0:
                part2 += 1
    print(f"{part2 = }")

ds = parse("01s", show=False)
d = parse("01", show=False)
part1(ds)
part1(d)

print()

part2(ds)
part2(d)
