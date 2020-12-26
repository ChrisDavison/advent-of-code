from prelude import *
from collections import defaultdict, Counter
from machine import Machine

data = [int(l) for l in open('input/05').read().split(',')]
print(len(data))
# print(data[:5])

m = Machine(data)
print(m.run())
# Part1

# Part2


