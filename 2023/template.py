from utility import lines, parse, Path, timed

SAMPLE = """ """

# DATA = Path("input/DAYNUM2").read_text()


@timed
def part1(data=SAMPLE):
    data = parser(data)
    pass


@timed
def part2(data=SAMPLE):
    data = parser(data)
    pass


def parser(data):
    return parse(data, str, lines, show=8)


part1()
# part1(DAYNUM2)
# part2()
# part2(DAYNUM2)
