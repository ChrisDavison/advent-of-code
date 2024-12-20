from rich import pretty
from utility import atoms, parse, mapt

pretty.install()

DEBUG = True


def run_program(computer):
    a, b, c, instructions = computer
    pointer = 0
    while pointer < len(instructions):
        op, val = instructions[pointer : pointer + 2]
        pointer += 2
        combo = {4: a, 5: b, 6: c}.get(val, val)
        match op:
            case 0:
                a = a >> combo
            case 6:
                b = a >> combo
            case 7:
                c = a >> combo
            case 1:
                b ^= val
            case 4:
                b ^= c
            case 2:
                b = combo % 8
            case 5:
                yield combo % 8
            case 3:
                if a:
                    pointer = val


# def part2sample():
#     """The program is bespoke and needs decompiling


def part2(computer):
    """The program is bespoke and needs decompiling

    Mine breaks down to...
        b = a%7
        b = b^2
        c = a/2^b
        a = a/2^3
        b = b^7
        b = b^c
        out b
        if not a, break

    ...simplified
        b = (a%8)^2
        c = a >> b
        a = a >> 3
        b ^= 7 ^ c
        out b
        if not a, break

    In other words, every loop the program outputs a single bit of A (in octal), so we want to work
    backwards (i.e. find the 'last' octal value, and then slowly build A up until we cover every digit).
    In this way, we slowly gain information. Otherwise, we would have to guess until we go all the way
    through the output and get the perfect match.
    """
    a = b = c = 0
    instructions = computer[3]
    print(f"{instructions}")
    aa = {0}
    for i in reversed(range(len(instructions))):
        tail = instructions[i:]
        aa = {(A << 3) + i for A in aa for i in range(8)}
        # Find which octal digits give the right output
        # Every time we get a new digit, 'attach' it to the old one
        # i.e. if a 5 gives the right final digit,
        # try all combinations 5x...
        # then after we find x, try all combinations 5xy
        # ... there may be multiple values of xy etc that give the right output bits
        aa = {a for a in aa if list(run_program((a, b, c, instructions))) == tail}
        octs = mapt(oct, aa)
        print(f"Semi-valid octs: {octs}")
        # break
        print()
    return min(aa)
    # out = list(run_program((0o7, b, c, instructions)))
    # print(f"{out=}")


def parse_computer(filename):
    parsed = parse(filename, atoms, show=0)
    a = b = c = 0
    instructions = []
    for line in parsed:
        match line:
            case ["Register", "A", val]:
                a = val
            case ["Register", "B", val]:
                b = val
            case ["Register", "C", val]:
                c = val
            case ["Program", *vals]:
                instructions = vals
    return a, b, c, instructions


sample = parse_computer("17s")
data = parse_computer("17")
print("part1 sample", ",".join(map(str, run_program(sample))))
print("part1 data", ",".join(map(str, run_program(data))))
print()

print("part2", part2(data))
