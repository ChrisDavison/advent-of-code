#!/usr/bin/env python3
import day01

def test_day01part1_examples():
    examples = [(12, 2), (14, 2), (1969, 654), (100756, 33583)]
    for (inp, out) in examples:
        assert out == day01.required(inp)

def test_day01_part2_examples():
    examples = [(14, 2), (1969, 966), (100756, 50346)]
    for (inp, out) in examples:
        assert out == day01.req_rec(inp)
