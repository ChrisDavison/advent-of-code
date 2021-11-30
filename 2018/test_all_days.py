from day01 import *
from day02 import *
from day03 import *
from day04 import *


# DAY 01 =====================================
def test_day01part1():
    def parse_example_day1(inputstr):
        return [int(i) for i in inputstr.split(",")]

    assert day01p1(parse_example_day1("+1,+1,+1"), start=0) == 3
    assert day01p1(parse_example_day1("+1,+1,-2"), start=0) == 0
    assert day01p1(parse_example_day1("-1,-2,-3"), start=0) == -6

def test_day01part2():
    def parse_example_day1(inputstr):
        return [int(i) for i in inputstr.split(",")]

    assert day01p2(parse_example_day1("+1,-1"), start=0) == 0
    assert day01p2(parse_example_day1("+3,+3,+4,-2,-4"), start=0) == 10
    assert day01p2(parse_example_day1("-6,+3,+8,+5,-6"), start=0) == 5
    assert day01p2(parse_example_day1("+7,+7,-2,-7,-4"), start=0) == 14


# DAY 02 =====================================
def test_day02part1():
    inputs = [
        "abcdef",
        "bababc",
        "abbcde",
        "abcccd",
        "aabcdd",
        "abcdee",
        "ababab"
    ]
    assert day02p1(inputs) == 12

def test_day02part2():
    inputs = [
        "abcde",
        "fghij",
        "klmno",
        "pqrst",
        "fguij",
        "axcye",
        "wvxyz"
    ]
    assert day02p2(inputs) == "fgij"


# DAY 03 =====================================
def test_day03_parse_claim():
    assert parse_claim("#1 @ 1,3: 4x4") == Claim(1, 1, 3, 4, 4)

def test_day03part1():
    EXAMPLE = [parse_claim(c) for c in [
            "#1 @ 1,3: 4x4",
            "#2 @ 3,1: 4x4",
            "#3 @ 5,5: 2x2"
    ]]
    assert day03p1(EXAMPLE) == 4

def test_day03part2():
    EXAMPLE = [parse_claim(c) for c in [
            "#1 @ 1,3: 4x4",
            "#2 @ 3,1: 4x4",
            "#3 @ 5,5: 2x2"
    ]]
    assert day03p2(EXAMPLE) == 3


def test_day04_parse_record():
    assert parse_guard_record("[1518-09-01 00:58] wakes up") == (1518, 9, 1, 0, 58, "awake")

def test_day04part1():
    example = [
        "[1518-11-01 00:00] Guard #10 begins shift",
        "[1518-11-01 00:05] falls asleep",
        "[1518-11-01 00:25] wakes up",
        "[1518-11-01 00:30] falls asleep",
        "[1518-11-01 00:55] wakes up",
        "[1518-11-01 23:58] Guard #99 begins shift",
        "[1518-11-02 00:40] falls asleep",
        "[1518-11-02 00:50] wakes up",
        "[1518-11-03 00:05] Guard #10 begins shift",
        "[1518-11-03 00:24] falls asleep",
        "[1518-11-03 00:29] wakes up",
        "[1518-11-04 00:02] Guard #99 begins shift",
        "[1518-11-04 00:36] falls asleep",
        "[1518-11-04 00:46] wakes up",
        "[1518-11-05 00:03] Guard #99 begins shift",
        "[1518-11-05 00:45] falls asleep",
        "[1518-11-05 00:55] wakes up"
    ]
    records = sorted([parse_guard_record(r) for r in example], key=lambda x: x[:5])

