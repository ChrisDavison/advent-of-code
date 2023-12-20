from collections import defaultdict, deque
from functools import lru_cache
import re
import pyperclip
import math

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


class Module:
    def __init__(self, label, prefix, targets):
        self.label = label
        self.prefix = prefix
        self.targets = targets

        if self.prefix == '%':
            self.memory = -1
        else:
            self.memory = {}

    def __repr__(self):
        return f"{self.prefix}{self.label} -> {self.targets} {self.memory}"

def parse(data=SAMPLE):
    start = []
    rules = dict()
    for line in data:
        left, right = line.split(' -> ')
        outs = [thing.strip() for thing in right.split(',')]
        if left == 'broadcaster':
            start = outs
        else:
            prefix = left[0]
            label = left[1:]
            rules[label] = Module(label, prefix, outs)
    
    for name, module in rules.items():
        for target in module.targets:
            if target in rules and rules[target].prefix == '&':
                rules[target].memory[name] = -1
    
    return rules, start


def part1(data=SAMPLE, n=1):
    rules, start_targets = parse(data)
    n_lo = n_hi = 0
    for i in range(n):
        n_lo += 1
        q = deque([('broadcaster', target, -1) for target in start_targets])

        while q:
            op, target, val = q.popleft()

            if val == 1:
                n_hi += 1
            else:
                n_lo += 1

            if target not in rules:
                # print(target, 'not in rules')
                continue

            module = rules[target]
            # print(f"{module = :}")

            if module.prefix == '%':
                if val == -1:
                    module.memory = -module.memory
                    for o in module.targets:
                        q.append((module.label, o, module.memory))
            elif module.prefix == '&':
                module.memory[op] = val
                outval = -1 if all(x == 1 for x in module.memory.values()) else 1
                for o in module.targets:
                    q.append((module.label, o, outval))
    return n_lo * n_hi


def part2():
    rules, start_targets = parse(DATA)
    n = 0
    (sends_to_rx,) = [module.label for label, module in rules.items()
                      if 'rx' in module.targets]

    cycle_lengths = {}
    seen = {label: 0 for label, module in rules.items() if sends_to_rx in module.targets}

    while True:
        q = deque([('broadcaster', target, -1) for target in start_targets])
        n += 1

        while q:
            op, target, val = q.popleft()


            if target not in rules:
                # print(target, 'not in rules')
                continue

            module = rules[target]
            # print(f"{module = :}")

            if module.label == sends_to_rx and val == 1:
                seen[op] += 1

                if op not in cycle_lengths:
                    cycle_lengths[op] = n
                else:
                    assert n == seen[op] * cycle_lengths[op]
        
                if all(seen.values()):
                    x = 1
                    for l in cycle_lengths.values():
                        x = x * l // math.gcd(x, l)
                    pyperclip.copy(x)
                    return x
            

            if module.prefix == '%':
                if val == -1:
                    module.memory = -module.memory
                    for o in module.targets:
                        q.append((module.label, o, module.memory))
            elif module.prefix == '&':
                module.memory[op] = val
                outval = -1 if all(x == 1 for x in module.memory.values()) else 1
                for o in module.targets:
                    q.append((module.label, o, outval))
