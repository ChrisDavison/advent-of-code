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

DATA = DAY(1)


# --- part 1
def f(data):
    s = 0
    for d in map(digits, data.splitlines()):
        s += d[0] * 10 + d[-1]
    print(s)
    return s


# assert f(TEST_INPUT) == 142
assert f(DATA) == 56465


# --- part 2
def f2(data):
    s = 0
    for d in map(digits_and_worddigits, data.splitlines()):
        s += d[0] * 10 + d[-1]
    print(s)
    return s


# assert f2(TEST_INPUT) == 142
# assert f2(TEST_INPUT_2) == 281
assert f2(DATA) == 55902
