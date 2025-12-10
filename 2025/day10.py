from collections import namedtuple
from utility import *

dtype = namedtuple('Day10', 'diagram switches voltages')

def pparse(filename):
    parsed = []
    for line in open("input/" + filename).read().splitlines():
        diagram, rest = line.split(' ', 1)
        diagram = [False if c == '.' else True for c in diagram[1:-1]]
        vstart = rest.find("{")
        voltages = tuple(int(v) for v in rest[vstart+1:-1].split(','))
        triggers = [tuple(map(int, chunk[1:-1].split(','))) for chunk in rest[:vstart-1].split(' ')]
        parsed.append(dtype(diagram,  triggers, voltages))
    return parsed


def trigger(diagram, switches):
    nd = diagram
    for switch in switches:
        nd[switch] = not nd[switch]
    return nd

def p(diagram):
    print(''.join('░' if not c else '▒' for c in diagram))


def solve_triggers(diagram, switches):
    start = [False] * len(diagram)
    pass

def part1(data):
    result = 0
    p(data[0].diagram)
    p(trigger(data[0].diagram, data[0].switches[0]))
    return result

def part2(data):
    result = 0
    return result

ds = pparse("10s")
dd = pparse("10")

header(10, 1)
print(part1(ds))
# print(part1(dd))

# header(10, 2)
# print(part2(ds))
# print(part2(dd))
