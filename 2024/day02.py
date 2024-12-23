from utility import *
import sys


def check(levels):
    samesign = all([x > 0 for x in d]) or all([x < 0 for x in d])
    if all([abs(x) in [1, 2, 3] for x in d]) and samesign:
        return True


if __name__ == "__main__":
    data = parse(sys.argv[1], ints)

    P = []
    s = 0
    for nums in data:
        d = [b - a for a, b in zip(nums, nums[1:])]
        if check(d):
            s += 1
    print(f"Part 1: {s}")

    P = []
    s = 0
    for nums in data:
        for j in range(len(nums)):
            subset = nums[:j] + nums[j + 1 :]
            d = [b - a for a, b in zip(subset, subset[1:])]
            if check(d):
                s += 1
                break
    print(f"Part 2: {s}")
