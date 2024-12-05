from collections import defaultdict
import re
from utility import *

SAMPLE = """47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"""
DATA = open("input/05").read()


def parse(data):
    rules = dict()
    rulestr, manual = paragraphs(data)
    for line in rulestr.splitlines():
        a, b = atoms(line)
        if a in rules:
            rules[a][b] = True
        else:
            rules[a] = {b: True}
        if b in rules:
            rules[b][a] = False
        else:
            rules[b] = {a: False}
    manuals = [atoms(line) for line in manual.splitlines()]
    return rules, manuals


def check(rules, line):
    all_valid = True
    for i, num in enumerate(line):
        if num not in rules:
            continue
        numrules = rules[num]
        if num not in rules:
            continue
        rest = line[i + 1 :]
        valid = True
        cont = [numrules.get(r, True) for r in rest]
        if not all(cont):
            all_valid = False
            break
    return all_valid


def part1(data):
    rules, manuals = parse(data)
    valids = [m for m in manuals if check(rules, m)]
    tot = sum(v[int(len(v) / 2)] for v in valids)
    print(f"Part1 {tot}")


def swap_till_valid(invalid, rules):
    invalid = list(invalid)
    while not check(rules, invalid):
        for i, num in enumerate(invalid):
            numrules = rules[num]
            false_indices = [
                j + i + 1
                for j, r in enumerate(invalid[i + 1 :])
                if not numrules.get(r, True)
            ]
            if not false_indices:
                continue
            values = [invalid[j] for j in false_indices]
            for j in false_indices[::-1]:
                invalid = invalid[:j] + invalid[j + 1 :]
            for v in values:
                invalid.insert(i, v)
            break
    return invalid


def part2(data):
    rules, manuals = parse(data)
    invalids = [m for m in manuals if not check(rules, m)]
    tot = sum(v[int(len(v) / 2)] for v in invalids)
    valids = [swap_till_valid(invalid, rules) for invalid in invalids]
    tot = sum(v[int(len(v) / 2)] for v in valids)
    print(f"Part2 {tot}")


# part1(SAMPLE)
part1(DATA)

# part2(SAMPLE)
part2(DATA)
