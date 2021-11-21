#!/usr/bin/env python3
from prelude import *
from intcode import IntCode
import logging

def run_with_replace(data, a, b):
    data[1] = a
    data[2] = b
    m = IntCode(data, level=logging.WARNING)
    m.run()
    return m.ops[0]


def part1(data):
    ans = run_with_replace(data.copy(), 12, 2)
    return ans


def part2(data, target=19690720):
    for i in range(100):
        for j in range(100):
            out = run_with_replace(data.copy(), i, j)
            if out == target:
                p2 = 100 * i + j
                return p2


if __name__ == "__main__":
    data = [int(d) for d in open('input/02').read().split(',')]
    timed("2019 2.1", part1, data)
    timed("2019 2.2", part2, data)
