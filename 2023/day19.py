from collections import defaultdict
import operator
import re
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


def predgen(predicatestr):
    m = re.findall(r'([xmas])([<>])(\d+)', predicatestr)[0]
    func = operator.lt if m[1] == '<' else operator.gt
    key = m[0]
    value = int(m[2])

    def inner(part):
        # print(f"{part = :} {key} {value}")
        return func(part[key], value)
    return inner


def acceptable(part, ruleset, curkey='in'):
    if curkey == 'A':
        return True
    if curkey == 'R':
        return False

    rules = ruleset[curkey]
    while rules:
        predicate, nextkey, pstr = rules[0]
        rules = rules[1:]
        if predicate(part):
            curkey = nextkey
            break
    return acceptable(part, ruleset, curkey)


def parse(data):
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
    return ruleset


ruleset = parse(DATA)
res = 0
for p in parts.splitlines():
    x, m, a, s = [int(v.split('=')[1]) for v in p[1:-1].split(',')]
    part = {'x': x, 'm': m, 'a': a, 's': s}
    if acceptable(part, ruleset):
        res += sum(part.values())
print(f"Part 1: {res}")


# part 2
# for every combination of 1..4000 for x, m, a, s
# count how many are acceptable
# i.e. instead for 4000 * 4000 * 4000 * 4000 parts, count acceptable

# we _DONT_ want to create these parts as that's going to be an obscene amount of calculation
# instead, consider the ACCEPT/REJECT at each stage as a binary tree
# count the number of inputs that would get into each path
def as_tree(rules):
    pass

# ruleset = parse(SAMPLE)
# for k, rules in ruleset.items():
#     print(k)
#     for r in rules:
#         print(f"\t{r[2]} -> {r[1]}")
