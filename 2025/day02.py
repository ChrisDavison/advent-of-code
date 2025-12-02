from utility import *

# data = parse("02s", positive_ints, lambda x: x.split(','), show=False)
data = parse("02", positive_ints, lambda x: x.split(','), show=False)

def is_invalid(num):
    ns = str(num)
    na = ns[:len(ns)//2]
    nb = ns[len(ns)//2:]
    return na == nb

def is_invalid_b(num):
    rx = re.compile(r'^(\d+)\1+$')
    return rx.match(str(num)) is not None


def run():
    inv = inv2 = 0
    for a, b in data:
        for val in range(a, b+1):
            if is_invalid(val):
                inv += val
            if is_invalid_b(val):
                inv2 += val
    print("part1:", inv)
    print("part2:", inv2)

if __name__ == "__main__":
    run()

# is_invalid_c(9998888999888)
