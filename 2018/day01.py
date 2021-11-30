from prelude import *

def gen_input():
    for line in Path("input/day01").read_text().splitlines():
        yield(int(line))


def day01p1(diffs, start):
    return start + sum(diffs)

def day01p2(diffs, start):
    reached = set()
    reached.add(start)
    for diff in itertools.cycle(diffs):
        start = start + diff
        if start in reached:
            return start
        reached.add(start)

if __name__ == "__main__":
    print(day01p1(gen_input(), start=0))
    print(day01p2(gen_input(), start=0))
