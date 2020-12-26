from prelude import *

data = [int(l) for l in open('input/01').read().splitlines()]

print(data[:5])

def required(mass):
    return np.floor(mass / 3) - 2

def req_rec(mass):
    total = 0
    while mass > 0:
        print(mass, total)
        mass = required(mass)
        if mass < 0:
            break
        total += mass
    return total

p1 = sum([required(m) for m in data])
print(p1)
clip.copy(p1)

p2 = sum([req_rec(m) for m in data])
print(int(p2))
clip.copy(int(p2))

req_rec(100756)
