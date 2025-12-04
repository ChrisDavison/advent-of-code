from utility import *

ds = Grid({(j, i):  col  for i, row in enumerate(parse("04s", show=False)) for j, col in enumerate(row)}, directions=directions8)
dd = Grid({(i, j):  col  for i, row in enumerate(parse("04", show=False)) for j, col in enumerate(row)}, directions=directions8)

def part1(grid):
    valid = set()
    for point in grid:
        n_nearby = sum(p != '.' for p in grid.neighbor_contents(point))
        # print(point, n_nearby)
        if grid[point] == '.':
            continue
        if n_nearby < 4:
            valid.add(point)
            grid[point] = 'x'
    print(len(valid))


part1(ds)
part1(dd)
# print()
# data.print()
