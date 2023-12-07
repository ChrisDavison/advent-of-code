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
@timed
def f(parsed):
    s = 0
    for d in parsed:
        s += d[0] * 10 + d[-1]
    return s

f(parse(1, digits, lines, show=0))
f(parse(1, digits_and_worddigits, lines, show=0))
