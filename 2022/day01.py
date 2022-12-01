example = """1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"""


def part1(data):
    return max(sum(row) for row in data)

def part2(data):
    sums = [sum(row) for row in data]
    return sum(sorted(sums)[-3:])


elves_ex = [[int(n) for n in row.split()] for row in example.split("\n\n")]
elves_data = [[int(n) for n in row.split()] for row in open("inputs/01").read().split("\n\n")]

print(part1(elves_ex))
print(part1(elves_data))

print(part2(elves_ex))
print(part2(elves_data))
