from prelude import *

def gen_input():
    for line in Path("input/day02").read_text().splitlines():
        yield(line)

def day02p1(inputs):
    "Multiply how many inputs have a double, multiplied by how many inputs have a triple."
    n_double = 0
    n_triple = 0
    for inp in inputs:
        c = Counter(inp)
        if 2 in c.values():
            n_double += 1
        if 3 in c.values():
            n_triple += 1
    return n_double * n_triple

def has_1_difference(a, b):
    return len(difference_positions(a, b)) == 1

def difference_positions(a, b):
    diff_positions = []
    if len(a) != len(b):
        return diff_positions
    for idx in range(len(a)):
        l1 = a[idx]
        l2 = b[idx]
        if l1 != l2:
            diff_positions.append(idx)
    return diff_positions

def day02p2(inputs):
    "Find 2 inputs that differ by 1 character, then return the string without the differing characters"
    # could do.. for i in inputs, for j in inputs[1:]... but that's an N2 algorithm
    # something smart with dictionaries?
    inputs = list(inputs)
    for (i, inp1) in enumerate(inputs):
        for (j, inp2) in enumerate(inputs[i+1:]):
            if has_1_difference(inp1, inp2):
                p = difference_positions(inp1, inp2)[0]
                return inp1[:p] + inp1[p+1:]

if __name__ == "__main__":
    print(day02p1(gen_input()))
    print(day02p2(gen_input()))
