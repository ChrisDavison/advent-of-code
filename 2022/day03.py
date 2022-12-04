ex = """vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"""

def priority(letter):
    ordDup = ord(letter)
    if letter.lower() == letter:
        ordDup -= ord('a') - 1
    else:
        ordDup -= ord('A') - 27
    return ordDup

def part1(data: str):
    tot = 0
    for row in data.splitlines():
        n = len(row)//2
        left = set(row[:n])
        right = set(row[n:])
        dup = list(left & right)[0]
        tot += priority(dup)
    return tot

def part2(data):
    lines = data.splitlines()
    groups = [lines[i:i+3] for i in range(0, len(lines), 3)]
    common_items = [list(set(a) & set(b) & set(c))[0] for a, b, c in groups]
    priorities = [priority(letter) for letter in common_items]
    # print(groups)
    # print(common_items)
    # print(priorities)
    return sum(priorities)

print(part1(ex))
print(part1(open("inputs/03").read()))

print(part2(ex))
print(part2(open("inputs/04").read()))
