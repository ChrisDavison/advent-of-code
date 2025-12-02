from utility import *

data = parse("02s", positive_ints, lambda x: x.split(','), show=False)
data = parse("02", positive_ints, lambda x: x.split(','), show=False)

@cache
def is_invalid(num):
    ns = str(num)
    na = ns[:len(ns)//2]
    nb = ns[len(ns)//2:]
    return na == nb

@cache
def is_invalid_b(num):
    rx = re.compile(r'^(\d+)\1+$')
    return rx.match(str(num)) is not None


inv = list()
inv2 = list()
for a, b in data:
    for val in range(a, b+1):
        if is_invalid(val):
            inv.append(val)
        if is_invalid_b(val):
            inv2.append(val)
print("part1:", sum(inv))
print("part2:", sum(inv2))

