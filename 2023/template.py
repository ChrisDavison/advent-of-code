from utility import *

SAMPLE = """ """

DATA = Path("input/DAYNUM2").read_text()

P = mapt(parse, SAMPLE.splitlines())
# G = as_grid(SAMPLE)

# part1
timer()


res = 0
timer(f"Part 1: {res}")
pyperclip.copy(int(res))

# part2
timer(reset=True)


res = 0
timer(f"Part 2: {res}")
pyperclip.copy(int(res))


