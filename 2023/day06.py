import re

sample = """Time:      7  15   30
Distance:  9  40  200"""

source = sample
source = open("input/day06").read()

data = list([[number for number in re.findall(r'\d+', line)]
        for line in source.splitlines()])

def f(tnum, dnum):
    s = 0
    for t in range(tnum):
        val = (-(t*t) + tnum*t - dnum)
        if val > 0:
            s += 1
    return s

prod = 1
for (time, record) in zip(*data):
    prod *= f(int(time), int(record))
print(prod)

tnum = int(''.join(x for x in data[0]))
dnum = int(''.join(x for x in data[1]))

print(f(tnum, dnum))
