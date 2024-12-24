from utility import *
from collections import namedtuple, deque
import time


state = dict()
connections = []
wires, connections = parse("24", section_by=paragraphs, show=0)

state = {wire: value for wire, value in parse(wires, atoms, show=0)}
# # # print(f"{state=}")

Rule = namedtuple("Rule", "command lhs rhs out")

connections = {
    c: Rule(rule, a, b, c) for a, rule, b, c in parse(connections, atoms, show=0)
}

outputs = {}
keys = deque(connections.keys())

while keys:
    # # print(f"({len(keys)}) {', '.join(keys)} ")
    key = keys.popleft()
    conn = connections[key]
    # time.sleep(0.1)
    if conn.lhs in outputs:
        lhs = outputs[conn.lhs]
        # # print(f"{lhs} in outputs")
    elif conn.lhs in state:
        lhs = state[conn.lhs]
        # # print(f"{conn.lhs} in wires")
    else:
        # we still need to solve LHS
        keys.append(conn.lhs)
        # re-add the current target because we didn't solve it
        keys.append(key)
        continue

    if conn.rhs in outputs:
        rhs = outputs[conn.rhs]
        # # print(f"{rhs} in outputs")
    elif conn.rhs in state:
        rhs = state[conn.rhs]
        # # print(f"{conn.rhs} in wires")
    else:
        # we still need to solve RHS
        keys.append(conn.rhs)
        # re-add the current target because we didn't solve it
        keys.append(key)
        continue

    # If we have both necessary connections, run the command
    # # print(f"SOLVING {conn.out} {key}")
    # # print(f"{lhs, rhs=}")
    # input()
    match conn.command:
        case "AND":
            outputs[key] = lhs & rhs
        case "XOR":
            outputs[key] = lhs ^ rhs
        case "OR":
            outputs[key] = lhs | rhs
    if conn.lhs in keys:
        keys.remove(conn.lhs)
    if conn.rhs in keys:
        keys.remove(conn.rhs)
    if key in keys:
        keys.remove(key)
    # print(f"{keys=}")

zkeys = sorted([k for k in outputs if k.startswith("z")], reverse=True)
print(int("".join(map(str, [outputs[k] for k in zkeys]))))
print(int("".join(map(str, [outputs[k] for k in zkeys])), 2))
