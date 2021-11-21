#!/usr/bin/env python3
from prelude import *
from collections import defaultdict, Counter
from intcode import IntCode
import logging

def part1(data):
    m = IntCode(data, inputs=[1], level=logging.WARNING)
    m.run()
    return m.last_output()


def part2(data):
    m = IntCode(data, inputs=[5], level=logging.WARNING)
    m.run()
    return m.last_output()


if __name__ == "__main__":

    data = [l for l in open('input/05').read().split(',')]
    timed("2019 5.1", part1, data)
    timed("2019 5.2", part2, data)

