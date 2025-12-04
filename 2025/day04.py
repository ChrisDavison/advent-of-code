from utility import *

data = Grid({(j, i):  col  for i, row in enumerate(parse("04s", show=False)) for j, col in enumerate(row)}, directions=directions8)
data = Grid({(i, j):  col  for i, row in enumerate(parse("04", show=False)) for j, col in enumerate(row)}, directions=directions8)
#data = parse("04")

# data.print()

valid = set()
for point in data:
    n_nearby = sum(p != '.' for p in data.neighbor_contents(point))
    # print(point, n_nearby)
    if data[point] == '.':
        continue
    if n_nearby < 4:
        valid.add(point)
        data[point] = 'x'
print(len(valid))

# print()
# data.print()
#
