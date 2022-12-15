from collections import defaultdict
from typing import *

ex = """$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"""

def parse(data: str) -> defaultdict[str, Set[str]]:
    dirs = defaultdict(set)
    current: List[str] = []
    for chunk in data.split('$ ')[1:]:
        lines = chunk.splitlines()
        rule = lines[0].strip()
        # print(rule)
        if rule.startswith('cd'):
            target = rule.split(' ')[1]
            if target == '..':
                current = current[:-1]
            else:
                current.append(rule.split(' ')[1])

        path = '/'.join(current)
        for line in lines[1:]:
            if line.startswith('dir'):
                continue
            size, name = line.split()
            dirs[path].add((int(size), name))
    return dirs

def part1(data):
    # walk/recreate the filetree and sum all their sizes
    dirs = parse(data)
    sizes = {}
    keys = set()
    for key in sorted(dirs.keys(), key=len)[::-1]:
        parent = '/'.join(key.split('/')[:-1])
        for size, file in dirs[key]:
            if key not in sizes:
                sizes[key] = 0
            sizes[key] += size
        children = {k for k in dirs.keys() if k != key and k.startswith(key)}
        print(f"{key} -- {dirs[key]} -- {children}")
        sizes[key] += sum(size for k in children for size, _name in dirs[k])
            # dirs[parent].add((size, file))
    print(sizes)
    return sum(v for k, v in sizes.items() if v < 100000)


def part2():
    pass

print(part1(ex))
print('----------------------')
print(part1(open('inputs/07').read()))

