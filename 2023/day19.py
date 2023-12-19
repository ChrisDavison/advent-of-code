from collections import defaultdict
from dataclasses import dataclass
import operator
import re
from pathlib import Path
from utility import first

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

data = DATA


@dataclass
class Part:
    x: int
    m: int
    a: int
    s: int


def predgen(predicatestr):
    m = re.findall(r'([xmas])([<>])(\d+)', predicatestr)[0]
    func = operator.lt if m[1] == '<' else operator.gt
    key = m[0]
    value = int(m[2])

    def inner(part):
        # print(f"{part = :} {key} {value}")
        return func(part[key], value)
    return inner


def acceptable(part, ruleset):
    print(part)
    print()
    curkey = 'in'
    while curkey not in 'AR':
        rules = ruleset[curkey]
        rulestidy = ' || '.join([f"{r[2]} -> {r[1]}" for r in rules])
        print(curkey, '\t', rulestidy)
        while rules:
            predicate, nextkey, pstr = rules[0]
            rules = rules[1:]
            print(f"\tif {pstr} -> {nextkey}", end=' ')
            if predicate(part):
                print(f"\tPASSED -> {nextkey}")
                curkey = nextkey
                break
            else:
                print(f"\tFAILED")
        # input()
    if curkey == 'A':
        acc.append(part['score'])
    print('='*80)
    # break



rules, parts = data.split('\n\n')
ruleset = defaultdict(list)
for line in rules.splitlines():
    key, rr = line[:-1].split('{')
    rs = []
    for rule in rr.split(','):
        if ':' in rule:
            predicatestr, nextkey = rule.split(':')
            predicate = predgen(predicatestr)
            rs.append((predicate, nextkey, predicatestr))
        else:
            predicate = lambda x: True
            nextkey = rule
            rs.append((predicate, nextkey, 'TRUE'))
    ruleset[key] = rs
acc = []

for p in parts.splitlines():
    x, m, a, s = [int(v.split('=')[1]) for v in p[1:-1].split(',')]
    part = {'x': x, 'm': m, 'a': a, 's': s, 'score': x + m + a + s}

    if acceptable(part, ruleset):
        acc.append(part)
    # break

res = sum(acc)
print(f"Part 1: {res}")

