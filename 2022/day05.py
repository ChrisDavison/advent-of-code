ex = """    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"""

def parse(data):
    # print(data)
    stack, orders = data.split("\n\n")
    stack = stack.splitlines()
    nstacks = len(stack[len(stack)-1].replace(' ', ''))
    stack = [s.replace('[', ' ').replace(']', ' ') for s in stack[:-1][::-1]]

    print('\n'.join(stack[::-1]))
    rules = []
    for line in orders.splitlines():
        parts = line.split()
        rules.append((int(parts[1]), int(parts[3]), int(parts[5])))

    stacks = [[] for i in range(nstacks)]
    for s in stack:
        s = s[::2]
        print(s)
        for i, thing in enumerate(s.rstrip()):
            stacks[i].append(thing)

    return stacks, rules

def part1(data):
    stacks, rules = parse(data)
    print(stacks)
    for (n, frm, to) in rules:
        print(stacks, n, frm, to)
        for i in range(n):
            val = stacks[frm - 1].pop()
            stacks[to - 1].append(val)
    print(stacks)

print('-' * 40)
part1(ex)
