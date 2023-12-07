import math as m
from pathlib import Path
from utility import *

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

DATA = Path('input/01').read_text()

# --- part 1
def f(parsed):
    s = 0
    for d in parsed:
        s += d[0] * 10 + d[-1]
    print(s)
    return s

# assert f(TEST_INPUT) == 142
parsed = parse(1, digits, lines, show=0)
assert f(parsed) == 56465

parsed = parse(1, digits_and_worddigits, lines, show=0)
# assert f2(TEST_INPUT) == 142
# assert f2(TEST_INPUT_2) == 281
assert f(parsed) == 55902
