ex = """mjqjpqmgbljsphdztnvjfqwrcgsmlb"""
ex2 = """bvwbjplbgvbhsrlpgdmjqwftvncz"""
ex3 = """nppdvjthqldpwncqszvftbrmjlhg"""
ex4 = """nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"""
ex5 = """zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"""

def part1(data):
    # 4 consecutive unique characters
    n = 4
    for i in range(len(data)-n):
        # print(data[i:i+4])
        if len(set(data[i:i+n])) == n:
            return i+n
    return -1

def part2(data):
    # 14 consecutive unique characters
    n = 14
    for i in range(len(data)-n):
        # print(data[i:i+4])
        if len(set(data[i:i+n])) == n:
            return i+n
    return -1


# print(ex)
print(part1(ex))
# print(part1(ex2))
# print(part1(ex3))
# print(part1(ex4))
# print(part1(ex5))
print(part1(open('inputs/06').read()))

print(part2(ex))
# print(part2(ex2))
# print(part2(ex3))
# print(part2(ex4))
# print(part2(ex5))
print(part2(open('inputs/06').read()))

