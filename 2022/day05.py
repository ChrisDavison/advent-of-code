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
    idx_to_stack = {i: int(c)-1 for i, c in enumerate(stack[len(stack) - 1]) if c != ' '}
    # print(len(idx_to_stack))
    # print(idx_to_stack)
    stack = [s.replace('[', ' ').replace(']', ' ') for s in stack[:-1][::-1]]

    # print('\n'.join(stack[::-1]))
    rules = []
    for line in orders.splitlines():
        parts = line.split()
        rules.append((int(parts[1]), int(parts[3]), int(parts[5])))

    stacks = [[] for i in range(nstacks)]
    for s in stack:
        # print('-', s, '-')
        for i, thing in enumerate(s):
            if thing not in ' []':
                stacks[idx_to_stack[i]].append(thing)

    return stacks, rules

def part1(data):
    stacks, rules = parse(data)
    # print(stacks)
    for (n, frm, to) in rules:
        # print(stacks, n, frm, to)
        for i in range(n):
            val = stacks[frm - 1].pop()
            stacks[to - 1].append(val)
    # print(stacks)
    lasts = [stack[len(stack)-1] for stack in stacks]
    print(''.join(lasts))


def part2(data):
    stacks, rules = parse(data)
    # print(stacks)
    for (n, frm, to) in rules:
        # print(stacks, n, frm, to)
        toadd = [stacks[frm - 1].pop() for i in range(n)][::-1]
        stacks[to - 1].extend(toadd)
    # print(stacks)
    lasts = [stack[len(stack)-1] for stack in stacks]
    print(''.join(lasts))


print('-' * 40)
part1(ex)
part1(open('inputs/05').read())

part2(ex)
part2(open('inputs/05').read())
