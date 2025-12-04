from utility import *

def parsegrid(s):
    return Grid({(j, i):  col  for i, row in enumerate(parse(s, show=False)) for j, col in enumerate(row)}, directions=directions8)


def part1(grid):
    valid = set()
    for point in grid:
        n_nearby = sum(p != '.' for p in grid.neighbor_contents(point))
        if grid[point] == '.':
            continue
        if n_nearby < 4:
            valid.add(point)
            grid[point] = 'x'
    print(len(valid))


def part2(grid):
    removals = 0
    changed = True
    while changed:
        changed = False
        for point in grid.findall('@'):
            n_nearby = sum(p != '.' for p in grid.neighbor_contents(point))
            if n_nearby < 4:
                removals += 1
                grid[point] = '.'
                changed = True
    print(removals)


part1(parsegrid("04s"))
part1(parsegrid("04"))
part2(parsegrid("04s"))
part2(parsegrid("04"))
