from collections import defaultdict
from pathlib import Path


SAMPLE = """px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"""
DATA = Path("input/19").read_text()


def acceptable(part, ruleset, curkey="in"):
    if curkey == "A":
        return True
    if curkey == "R":
        return False
    rules, fallback = ruleset[curkey]
    while rules:
        letter, op, value, nextkey = rules[0]
        rules = rules[1:]
        if op == ">" and part[letter] > value:
            return acceptable(part, ruleset, nextkey)
        elif op == "<" and part[letter] < value:
            return acceptable(part, ruleset, nextkey)
    return acceptable(part, ruleset, fallback)


def parse(data):
    rules, parts = data.split("\n\n")
    ruleset = defaultdict(list)
    for line in rules.splitlines():
        key, rr = line[:-1].split("{")
        rs = []
        fallback_key = rr.split(",")[-1]
        for rule in rr.split(",")[:-1]:
            predicatestr, nextkey = rule.split(":")
            letter = predicatestr[0]
            op = predicatestr[1]
            value = int(predicatestr[2:])
            rs.append((letter, op, value, nextkey))
        ruleset[key] = (rs, fallback_key)
    return ruleset, parts


ruleset, parts = parse(DATA)
res = 0
for p in parts.splitlines():
    x, m, a, s = [int(v.split("=")[1]) for v in p[1:-1].split(",")]
    part = {"x": x, "m": m, "a": a, "s": s}
    if acceptable(part, ruleset):
        res += sum(part.values())
print(f"Part 1: {res}")


# part 2
# for every combination of 1..4000 for x, m, a, s
# count how many are acceptable
# i.e. instead for 4000 * 4000 * 4000 * 4000 parts, count acceptable
# ----------
# we _DONT_ want to create these parts as that's going to be an obscene amount
# of calculation instead, consider the ACCEPT/REJECT at each stage as a binary
# tree and count the number of inputs that would get into each path
def count(ranges, curkey="in"):
    if curkey == "R":
        return 0
    if curkey == "A":
        product = 1
        for lo, hi in ranges.values():
            product *= hi - lo + 1
        return product
    total = 0
    rules, fallback = ruleset[curkey]
    for letter, op, value, nextkey in rules:
        lo, hi = ranges[letter]  # Currently POSSIBLY viable ranges for letter
        if op == "<":
            trues = (lo, value - 1)
            falses = (value, hi)
        else:
            trues = (value + 1, hi)
            falses = (lo, value)
        if trues[0] <= trues[1]:
            copy = dict(ranges)
            copy[letter] = trues
            total += count(copy, nextkey)
        if falses[0] <= falses[1]:
            ranges = dict(ranges)
            ranges[letter] = falses
        else:
            break
    else:
        total += count(ranges, fallback)
    return total


res = count({key: (1, 4000) for key in "xmas"})
print(f"Part 2: {res}")
