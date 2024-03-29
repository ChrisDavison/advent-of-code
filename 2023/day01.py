from utility import Path, parse, digits, lines, digits_and_worddigits, timed

TEST_INPUT = """1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"""

TEST_INPUT_2 = """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"""

DATA = Path("input/01").read_text()

s = 0
for line in DATA.splitlines():
    d = digits(line)
    s += d[0] * 10 + d[-1]
print("Part 1: ", s)


s = 0
for line in DATA.splitlines():
    d = digits_and_worddigits(line)
    s += d[0] * 10 + d[-1]
print("Part 1: ", s)
