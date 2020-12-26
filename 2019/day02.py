from prelude import *
from machine import Machine

def run_with_replace(data, a, b):
    data[1] = a
    data[2] = b
    m = Machine(data)
    ans = m.run()
    return ans


def part1(data):
    ans = run_with_replace(data.copy(), 12, 2)
    print(ans)
    clip.copy(ans)
    

def part2(data, target=19690720):
    for i in range(100):
        for j in range(100):
            out = run_with_replace(data.copy(), i, j)
            if out == target:
                p2 = 100 * i + j
                print(f"{p2}")
                clip.copy(f"{p2}")
                return 


if __name__ == "__main__":
    data = [int(d) for d in open('input/02').read().split(',')]
    part1(data)
    part2(data)

