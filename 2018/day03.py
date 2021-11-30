from dataclasses import dataclass
from prelude import *
import sys

@dataclass
class Claim:
    who: str
    x: int
    y: int
    width: int
    height: int

def gen_input():
    for line in Path("input/day03").read_text().splitlines():
        yield parse_claim(line)

def parse_claim(claim):
    m = re.search("#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)", claim) 
    if m:
        who, x, y, w, h = [int(m.group(g)) for g in [1, 2, 3, 4, 5]]
        return Claim(who, x, y, w, h)

def claimed_grid(claims):
    grid = defaultdict(list)
    for claim in claims:
        for i in range(claim.width):
            for j in range(claim.height):
                grid[(claim.x+i,claim.y+j)].append(claim.who)
    return grid

def day03p1(claims):
    return len([v for v in claimed_grid(claims).values() 
        if len(v) > 1])

def day03p2(claims):
    claims = list(claims)
    grid = claimed_grid(claims)
    for claim in claims:
        coords = [(claim.x+i, claim.y+j) 
                for i in range(claim.width) 
                for j in range(claim.height)]
        if all([grid[c] == [claim.who] for c in coords]):
            return claim.who
    return "NO MATCHING CLAIM??!"


if __name__ == "__main__":
    print(day03p1(gen_input()))
    print(day03p2(gen_input()))
