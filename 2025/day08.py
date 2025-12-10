from utility import *
from functools import reduce

ds = list(enumerate(parse("08s",positive_ints)))
dd = list(enumerate(parse("08",positive_ints, show=False)))

def part1(data, lim):
    distances = sorted([(distance(p1, p2), i, j) for i, p1 in data for j, p2 in data if i != j])
    boxes = []
    for _ in range(lim):
        # print()
        # print(f"{boxes = }")
        top = None
        for _, l, r in distances:
            if not any(l in bb and r in bb for bb in boxes):
                top = (l, r)
                break
        l_in = r_in = -1
        # print(f"{top = }")
        assert top is not None
        for i, b in enumerate(boxes):
            # print(f"{b = }")
            if top[0] in b:
                l_in = i

            if top[1] in b:
                r_in = i

        # print(f"{l_in = } {r_in = }")
        # print(f"{boxes = }")
        # print(f"{len(boxes) = }")
        if l_in >=0 and r_in >= 0:
            boxes[l_in].extend(boxes[r_in])
            del boxes[r_in]
        elif l_in >= 0:
            boxes[l_in].append(top[1])
        elif r_in >= 0:
            boxes[r_in].append(top[0])
        else:
            # create new box
            boxes.append([top[0], top[1]])
    
    # print("-"*40)
    # for b in boxes:
    #     print(f"{b = } {len(b)}")
    #
    lens = sorted([len(b) for b in boxes])
    # print(f"{lens = }")
    print(reduce(lambda acc, x: x * acc, lens[-3:], 1))

def part2(data):
    pass

part1(ds, lim=9)
part1(dd, lim=999)

