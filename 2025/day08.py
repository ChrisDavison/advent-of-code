from utility import *
from functools import reduce

ds = list(enumerate(parse("08s",positive_ints, show=False)))
dd = list(enumerate(parse("08",positive_ints, show=False)))

def part1(data, lim):
    distances = sorted([(distance(p1, p2), i, j) for (i, p1), (j, p2) in combinations(data, 2)])
    boxes = []
    for (_d, l, r) in distances[:lim]:
        l_in = r_in = -1
        for i, b in enumerate(boxes):
            if l in b:
                l_in = i
            if r in b:
                r_in = i
        if l_in == r_in and l_in != -1:
            continue
        if l_in >= 0 and r_in >= 0:
            boxes[l_in].extend(boxes[r_in])
            del boxes[r_in]
        elif l_in >= 0:
            boxes[l_in].append(r)
        elif r_in >= 0:
            boxes[r_in].append(l)
        else:
            # create new box
            boxes.append([l, r])
    
    lens = sorted([len(b) for b in boxes])
    print(reduce(lambda acc, x: x * acc, lens[-3:], 1))

def part2(data):
    distances = sorted([(distance(p1, p2), i, j) for (i, p1), (j, p2) in combinations(data, 2)])
    boxes = []
    for (_d, l, r) in distances:
        l_in = r_in = -1
        for i, b in enumerate(boxes):
            if l in b:
                l_in = i
            if r in b:
                r_in = i
        if l_in == r_in and l_in != -1:
            continue
        if l_in >= 0 and r_in >= 0:
            boxes[l_in].extend(boxes[r_in])
            del boxes[r_in]
        elif l_in >= 0:
            boxes[l_in].append(r)
        elif r_in >= 0:
            boxes[r_in].append(l)
        else:
            # create new box
            boxes.append([l, r])
        if len(boxes[0]) == len(data):
            l = data[l]
            r = data[r]
            print(l[1][0], r[1][0], l[1][0] * r[1][0])
            break
    

part1(ds, lim=10)
part1(dd, lim=1000)

part2(ds)
part2(dd)
