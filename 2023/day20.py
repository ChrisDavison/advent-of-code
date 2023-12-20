from collections import defaultdict, deque
from functools import lru_cache
import re

SAMPLE = """broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a""".splitlines()

SAMPLE2 = """broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output""".splitlines()
DATA = [line.strip() for line in open("input/20").readlines()]


def parse(data=SAMPLE):
    rules = dict()
    constate = defaultdict(list)
    ffstate = defaultdict(list)
    q = deque([('broadcaster', 0)])
    for line in data:
        i, o = line.split(' -> ')
        prefix = i[0]
        label = i[1:]
        o = [out.strip() for out in o.split(',')]
        if i == 'broadcaster':
            labels = [thing.strip() for thing in o]
            rules['broadcaster'] = ('broadcast', labels)
            continue
        if prefix == '&':
            # conjunction labels should track how many inputs they need
            if current := constate.get(label, None):
                constate[label] = (current[0]+1, [])
            else:
                constate[label] = (1, [])
        else:
            ffstate[label] = -1
        rules[label] = (prefix, o)
    return rules, q, constate, ffstate


def step(rules, q, constate, ffstate):
    n_lo = n_hi = 0
    n_lo += 1
    q = q.copy()
    while q:
        target, val = q.popleft()
        if target not in rules:
            print(target, 'not in rules')
            continue
        prefix, out = rules[target]
        if prefix == 'broadcast':
            for o in out:
                # print(f"broadcaster -low-> {o}")
                q.append((o, -1))
                n_lo += 1
        elif prefix == '%':
            if val == 1:
                # print(f"\t{target} -high-> NOOP")
                continue
            for o in out:
                q.append((o, -ffstate[target]))
                ss = '-low' if -ffstate[target] == -1 else '-high'
                # print(f"{target} {ss}-> {o}")
            ffstate[target] = -ffstate[target]
            if -ffstate[target] == -1:
                n_lo += len(out)
            else:
                n_hi += len(out)
        elif prefix == '&':
            n, conq = constate[target]
            conq.append(val)
            # print(f"{conq = :}")
            if len(conq) == n:
                # if we've got a full queue for this conjunction node,
                # process it
                outval = -val
                if n > 1:
                    outval = -1 if all(x == 1 for x in conq) else 1
                for o in out:
                    q.append((o, outval))
                if outval == 1:
                    n_hi += len(out)
                else:
                    n_lo += len(out)
                ss = '-low' if outval == -1 else '-high'
                # print(f"{target} {ss}-> {out}")
                constate[target] = (n, [])
            else:
                # just add the signal to the queue
                constate[target] = (n, conq + [val])
                # print(f"{target} acc append {val}")
    return rules, q, constate, ffstate, n_lo, n_hi


def part1(data=SAMPLE):
    rules, q, constate, ffstate = parse(data)
    n_lo = n_hi = 0
    for i in range(1000):
        rules, _, constate, ffstate, lo, hi = step(rules, q, constate, ffstate)
        n_lo += lo
        n_hi += hi
    print(f"{n_lo = :}, {n_hi = :}")
    print(f"Part 1: {n_lo * n_hi = :}")
