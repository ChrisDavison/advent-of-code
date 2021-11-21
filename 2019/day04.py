#!/usr/bin/env python3
from prelude import *
from collections import defaultdict

def p1_valid(pw):
    digits=[c for c in str(pw)]
    prev = digits[0]
    found_double = False
    for digit in digits[1:]:
        if digit < prev:
            return False
        if digit == prev:
            found_double = True
        prev = digit
    return found_double

def p2_valid(pw):
    pw = str(pw)
    prev = pw[0]
    found_double = False
    runs = defaultdict(lambda: 0)
    runs[prev] += 1
    for digit in pw[1:]:
        if digit < prev:
            return False
        if digit == prev:
            found_double = True
        else:
            if runs[prev] > 2:
                runs[prev] = 0
        prev = digit
        runs[digit] += 1
    return any(n == 2 for _, n in runs.items())

def part1(lower, upper):
    tot = sum([1 for poss in range(lower, upper) if p1_valid(poss)])
    return tot

def part2(lower, upper):
    tot = sum([1 for poss in range(lower, upper) if p2_valid(poss)])
    return tot


data = "136760-595730"
lower, upper = [int(i) for i in data.split("-")[:2]]
tot1, tot2 = 0, 0

timed("2019 4.1", part1, lower, upper)
timed("2019 4.2", part2, lower, upper)

