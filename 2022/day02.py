ex = """A Y
B X
C Z"""


def parse(data: str):
    return [row.split(' ') for row in data.splitlines() if row]


def result(opponent, you):
    return {
        'A': {'X': 3, 'Y': 6, 'Z': 0},
        'B': {'X': 0, 'Y': 3, 'Z': 6},
        'C': {'X': 6, 'Y': 0, 'Z': 3},
    }[opponent][you]


def should_pick(opponent, you):
    return {
        'X': {'A': 'Z', 'B': 'X', 'C': 'Y'},
        'Y': {'A': 'X', 'B': 'Y', 'C': 'Z'},
        'Z': {'A': 'Y', 'B': 'Z', 'C': 'X'},
    }[you][opponent]

def object_score(you):
    return {'X': 1, 'Y': 2, 'Z': 3}[you]


def part1(data):
    data = parse(data)
    return sum(result(op, you) + object_score(you) for op, you in data)

def part2(data):
    data = parse(data)
    tot = 0
    for op, you in data:
        # print(result(op, should_pick(op, you)), should_pick(op, you))
        tot += result(op, should_pick(op, you)) + object_score(should_pick(op, you))
    return tot
    # return sum(result(op, you) + object_score(you) for op, you in data)

print(part1(ex))
print(part1(open("inputs/02").read()))
print(part2(ex))
print(part2(open("inputs/02").read()))
