from utility import *
from collections import namedtuple, deque
import sys

Rule = namedtuple("Rule", "command lhs rhs out")


def part1(connections, state):
    outputs = {}
    keys = deque(connections.keys())

    while keys:
        key = keys.popleft()
        conn = connections[key]
        if conn.lhs in outputs:
            lhs = outputs[conn.lhs]
        elif conn.lhs in state:
            lhs = state[conn.lhs]
        else:
            keys.append(conn.lhs)
            keys.append(key)
            continue

        if conn.rhs in outputs:
            rhs = outputs[conn.rhs]
        elif conn.rhs in state:
            rhs = state[conn.rhs]
        else:
            keys.append(conn.rhs)
            keys.append(key)
            continue

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

    zkeys = sorted([k for k in outputs if k.startswith("z")], reverse=True)
    val = int("".join(map(str, [outputs[k] for k in zkeys])), 2)
    print(f"part1 ({sys.argv[1]}): {val}")


if __name__ == "__main__":
    state = dict()
    connections = []
    wires, connections = parse(sys.argv[1], section_by=paragraphs, show=0)

    state = {wire: value for wire, value in parse(wires, atoms, show=0)}

    connections = {
        c: Rule(rule, a, b, c) for a, rule, b, c in parse(connections, atoms, show=0)
    }

    part1(connections, state)
    part2(connections, state)
