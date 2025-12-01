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


def inc(start, value, direction):
    final = 0
    does_wrap = False
    rem = 0
    msg = ""
    match direction:
        case 'R':
            after = start + value
            does_wrap = after > 100
            final = after % 100
            rem = value - (100 - start)
            msg = f"{start:>2d} +{value:<2d} => {final:<2d}"
        case 'L':
            after = start - value
            does_wrap = after < 0
            final = after % 100
            does_wrap = (value > start) and start != 0
            rem = value - start
            msg = f"{start:>2d} -{value:<2d} => {final:>2d}"
    zeroes = 1 if final == 0 else 0
    if final == 0:
        rem -= 1

    wraps = 0
    if does_wrap:
        sign= 1 if rem > 0 else -1
        wraps = int(sign*rem / 100)
        print(f"{msg}\t{zeroes}+{wraps}", end=" ")
    else:
        zstr = f"{zeroes}" if zeroes else ""
        print(f"{msg}\t{zstr}   ", end=" ")

    return final, zeroes+wraps


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
