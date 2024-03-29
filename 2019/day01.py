#!/usr/bin/env python3
from prelude import *

data = [int(l) for l in open('input/01').read().splitlines()]

# print(data[:5])

def required(mass):
    return np.floor(mass / 3) - 2

def req_rec(mass):
    total = 0
    while mass > 0:
        # print(mass, total)
        mass = required(mass)
        if mass < 0:
            break
        total += mass
    return total


timed("2019 1.1", lambda: sum([required(m) for m in data]))
timed("2019 1.2", lambda: sum([req_rec(m) for m in data]))
