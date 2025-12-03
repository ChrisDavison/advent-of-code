from operator import indexOf
from utility import *
import numpy as np
import tqdm

ds = parse("03s", digits, show=False)
dd = parse("03", digits, show=False)


def part1(data):
    part1 = 0
    for line in data:
        linemax = 0
        for i, dig in enumerate(line[:-1]):
            num = dig * 10 + max(line[i + 1 :])
            if num > linemax:
                linemax = num
        part1 += linemax
    print("part1", chalk.red(part1))




def to_num(digits):
    n = len(digits)
    powered = [d * 10 ** (n - i - 1) for i, d in enumerate(digits)]
    return sum(powered)


def max_with_index(values):
    idx = np.argmax(values)
    return values[idx], idx


def max_noncontiguous_sequence(digits, found=None, length=12):
    # This searches for the largest number between the index of the previous
    # largest number and the end of the list
    # e.g. if we have
    # [1, 1, ... 1000x 1, 9, 1, 1]
    # it will find 9, and only use [1, 1] as the NEXT list to work through
    if not found:
        # first entry into the function
        found = []

    if length == 0:
        # we have enough 'found'
        return to_num(found)

    # can't just do negative final index as [:-0] gives empty list
    subset = digits[:len(digits)-(length-1)]
    val_max, pos_max = max_with_index(subset)
    return max_noncontiguous_sequence(
        digits[pos_max + 1 :], found + [val_max], length - 1
    )


def part2(data):
    tot = sum(max_noncontiguous_sequence(line, None, 12) for line in data)
    print("part2", chalk.red(tot))


part1(ds)
part1(dd)

print(chalk.green("·" * 80))

part2(ds)
part2(dd)
