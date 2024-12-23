from utility import paragraphs, atoms, parse
import sys


def parse_ruleset(filename):
    rules = dict()
    rulestr, manual = parse(filename, show=0, section_by=paragraphs)
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
        cont = [numrules.get(r, True) for r in rest]
        if not all(cont):
            all_valid = False
            break
    return all_valid


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


def part1(rules, manuals):
    valids = [m for m in manuals if check(rules, m)]
    tot = sum(v[int(len(v) / 2)] for v in valids)
    print(f"Part1 {tot}")


def part2(rules, manuals):
    invalids = [m for m in manuals if not check(rules, m)]
    tot = sum(v[int(len(v) / 2)] for v in invalids)
    valids = [swap_till_valid(invalid, rules) for invalid in invalids]
    tot = sum(v[int(len(v) / 2)] for v in valids)
    print(f"Part2 {tot}")


if __name__ == "__main__":
    part1(*parse_ruleset(sys.argv[1]))
    part2(*parse_ruleset(sys.argv[1]))
