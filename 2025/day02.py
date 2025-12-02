from utility import *

data = parse("02s", positive_ints, lambda x: x.split(','), show=False)
data = parse("02", positive_ints, lambda x: x.split(','), show=False)

@cache
def is_invalid(num):
    ns = str(num)
    na = ns[:len(ns)//2]
    nb = ns[len(ns)//2:]
    return na == nb



inv = list()
for a, b in data:
    for val in range(a, b+1):
        if is_invalid(val):
            inv.append(val)
print("part1:", sum(inv))
