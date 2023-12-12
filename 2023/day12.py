from utility import *

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
    pass


@timed
def part2(data=SAMPLE):
    data = parser(data)
    pass


def parser(data):
    out = []
    def parse_line(l):
        s, c = l.split(' ')
        c = ints(c)
        return ([ch for ch in s], c)

    return parse(data, parse_line, lines)
        
part1()
# part2()
