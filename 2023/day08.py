import sys, os
sys.path.insert(0, os.path.expanduser("~/code/advent-of-code/"))
from utility import *

SAMPLE = """RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"""

SAMPLE2 = """LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"""

@timed
def part1(data: Union[int, str]=SAMPLE):
    instructions, routes, _ = parser(data)
    n = 0
    current = "AAA"
    for instruction in cycle(instructions):
        current = routes[current][instruction]
        n += 1
        if current == 'ZZZ':
            break
    return n


@timed
def part2(data: Union[int, str]=SAMPLE):
    instructions, routes, n_instructions = parser(data)
    current = [k for k in routes.keys() if k[2] == "A"]
    cycles = [find_cycle(instructions, routes, start) for start in current]
    return m.lcm(*cycles, n_instructions)


def find_cycle(instructions, routes, start):
    current = start
    # need to also factor in that while I might have found a point before, I
    # may take a different direction this time. So need to find a multiple of
    # the cycle length and the number of instructions
    n = 0
    for instruction in cycle(instructions):
        current = routes[current][instruction]
        n += 1  # number of steps taken
        if current[2] == 'Z':
            return n
    # The only way we can get here is if we've not got any instructions
    return 0


def parser(data):
    stuff = parse(data, lambda x: re.findall(r"([\d\w]+)", x), lines, show=0)
    instructions = [0 if d == "L" else 1 for d in stuff[0][0]]
    m = dict()
    for line in stuff[2:]:
        a, b, c = line[0], line[1], line[2]
        m[a] = (b, c)
    return instructions, m, len(instructions)


part1(8)
part2(8)
