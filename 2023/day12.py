from utility import lines, parse, timed, Path, ints, re, mapt, permutations, cat
from itertools import product

SAMPLE = """???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"""

DATA = Path("input/12").read_text()


@timed
def part1(data=SAMPLE):
    data = parser(data)
    s = 0
    for (pattern, to_match) in data:
        n = get_permutations(pattern, to_match)
        s += n
        # print(pattern, to_match, n)
    return s


@timed
def part2(data=SAMPLE):
    data = parser(data)
    pass


def parser(data):
    def parse_line(line):
        s, c = line.split(' ')
        c = ints(c)
        return (s, c)

    return parse(data, parse_line, lines, show=0)


def count_springs(line):
    if isinstance(line, list):
        line = cat(line)
    return mapt(len, re.findall('#+', line))


def get_permutations(line, to_match):
    q = [m.start() for m in re.finditer(r'\?', line)]
    n = 0
    for permut in product([True, False], repeat=len(q)):
        l2 = list(line)
        for i, p in enumerate(permut):
            l2[q[i]] = '#' if p else '.'
        if count_springs(l2) == to_match:
            n += 1
    return n



print(count_springs(list('.##.##.###')))
print(get_permutations('.??.??.###', (2, 2, 3)))
part1()
part1(12)
# part2()
