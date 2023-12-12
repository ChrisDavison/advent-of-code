from utility import lines, parse, timed, Path, ints, re, mapt, permutations, cat, datetime, cache, timer, defaultdict
from itertools import product
import pyperclip

timer()

SAMPLE = """???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"""

DATA = Path("input/12").read_text()

timer()

D = []
for line in DATA.splitlines():
    s, c = line.split(' ')
    c = ints(c)
    D.append((s, c))
timer('Parse')


@cache
def count(p, s): 
    if not p:
        return 1 if not s else 0
    if not s:
        return 0 if "#" in p else 1

    result = 0

    if p[0] in ".?":
        result += count(p[1:], s)

    if p[0] in "#?":
        if s[0] <= len(p) and "." not in p[:s[0]] and (s[0] == len(p) or p[s[0]] != '#'):
            result += count(p[s[0] + 1:], s[1:])

    return result


s: int = 0
for (pattern, to_match) in D:
    s += count(pattern + '.', to_match)
timer(f"Part 1: {s}")
input("Enter to copy to clipboard")
pyperclip.copy(s)

timer(reset=True)
s: int = 0
for (p, to_match) in D:
    s += count('?'.join([p, p, p, p, p]), to_match * 5)
timer(f"Part 2: {s}")
input("Enter to copy to clipboard")
pyperclip.copy(s)

