from prelude import *


def day02():
    data = [int(d) for d in open('input/02').read().split(',')]
    part1(data.copy())
    part2(data)

def part1(data):
    data[1] = 12
    data[2] = 2
    p1 = run(data)
    print(p1)
    clip.copy(p1)

def part2(data, target=19690720):
    for i in range(100):
        for j in range(100):
            data2 = data.copy()
            data2[1] = i
            data2[2] = j
            out = run(data2)
            if out == target:
                p2 = 100 * i + j
                print(f"{p2}")
                clip.copy(f"{p2}")
                return

def run(opcodes, position=0):
    if opcodes[position] == 99:
        return opcodes[0]
    op = opcodes[position]
    a = opcodes[opcodes[position+1]]
    b = opcodes[opcodes[position+2]]
    c = opcodes[position+3]
    if op == 1:
        opcodes[c] = a + b
    else:
        opcodes[c] = a * b
    return run(opcodes, position+4)

