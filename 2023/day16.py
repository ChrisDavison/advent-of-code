from utility import *

SAMPLE = """ """
DATA = Path("input/16").read_text()

P = []
for line in SAMPLE.splitlines():
    P.append(line)

# part1
timer()

# ... do stuff ...

res = 0
timer(f"Part 1: {res}")
pyperclip.copy(int(res))

# part2
timer(reset=True)

# ... do stuff ...

res = 0
timer(f"Part 2: {res}")
pyperclip.copy(int(res))


