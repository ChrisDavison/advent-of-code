from utility import *
from heapq import heappop, heappush

timer(reset=True)

SAMPLE = """2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"""

DATA = Path("input/17").read_text()

P = np.array(as_grid(DATA), dtype=int)
# print(P)


# Pathfinding
# No more than three blocks in the same direction
# Minimise sum
# part1
timer(reset=True)


def step(cost, r, c, dr, dc, steps):
    nr = r + dr
    nc = c + dc
    if 0 <= nr < len(P) and 0 <= nc < len(P[0]):
        heappush(pq, (cost + P[nr][nc], nr, nc, dr, dc, steps + 1))


targetr, targetc = len(P) - 1, len(P[0]) - 1
seen = set()
# heat, r, c, dr, dc, s
pq = [(0, 0, 0, 0, 0, 0)]
res = 0
while pq:
    cost, r, c, dr, dc, s = heappop(pq)
    if (r, c) == (targetr, targetc):
        res = cost
        break
    if (r, c, dr, dc, s) in seen:
        continue
    seen.add((r, c, dr, dc, s))
    if s < 3 and (dr, dc) != (0, 0):
        # Continue current direction
        step(cost, r, c, dr, dc, s)
    for ndr, ndc in directions4:
        # Skip the 'straight' case, as it's covered above
        # and backtracking, as we've obviously already done it
        if (ndr, ndc) == (dr, dc) or (ndr, ndc) == (-dr, -dc):
            continue
        step(cost, r, c, ndr, ndc, 0)
timer(f"Part 1: {res}")
pyperclip.copy(int(res))

# part2
timer(reset=True)
seen = set()
# heat, r, c, dr, dc, s
pq = [(0, 0, 0, 0, 0, 0)]
res = 0
while pq:
    cost, r, c, dr, dc, s = heappop(pq)
    if (r, c) == (targetr, targetc) and s >= 4:
        res = cost
        break
    if (r, c, dr, dc, s) in seen:
        continue
    seen.add((r, c, dr, dc, s))
    if s < 10 and (dr, dc) != (0, 0):
        # Continue current direction
        step(cost, r, c, dr, dc, s)
    if s >= 4 or (dr, dc) == (0, 0):
        # Only turn if we've moved at least 4 steps
        for ndr, ndc in directions4:
            # Skip the 'straight' case, as it's covered above
            # and backtracking, as we've obviously already done it
            if (ndr, ndc) == (dr, dc) or (ndr, ndc) == (-dr, -dc):
                continue
            step(cost, r, c, ndr, ndc, 0)
timer(f"Part 2: {res}")
pyperclip.copy(int(res))
