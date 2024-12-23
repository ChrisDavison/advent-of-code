import re
import sys
from utility import *


if __name__ == "__main__":
    source = the(parse(sys.argv[1], section_by=paragraphs))

    P = []
    m = re.findall(r"mul\((\d+),(\d+)\)", source)
    s = 0
    do = True
    for group in m:
        x, y = int(group[0]), int(group[1])
        s += x * y
    print(f"Part 1: {s}")

    m = re.findall(r"(don't\(\)|do\(\)|mul\((\d+),(\d+)\))", source)
    s = 0
    do = True
    for group in m:
        match group[0]:
            case "don't()":
                do = False
            case "do()":
                do = True
            case default:
                if do:
                    x, y = int(group[1]), int(group[2])
                    s += x * y
    print(f"Part 2: {s}")
