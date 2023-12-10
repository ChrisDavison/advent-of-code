from utility import parse, ints, lines, timed, Path


SAMPLE = """0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"""

DATA = Path("input/09").read_text()

@timed
def do(data=SAMPLE, rev_line=False):
    data = parser(data)
    s = 0
    for line in data:
        if rev_line:
            line = line[::-1]
        final = line[-1]
        while any(value != 0 for value in line):
            line = [y - x for x, y in zip(line, line[1:])]
            final += line[-1]
        s += final
    return s


def parser(data):
    return parse(data, ints, lines, show=0)


do(9)
do(9, rev_line=True)
