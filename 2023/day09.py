from utility import *

SAMPLE = """ """

DATA = Path("input/09").read_text()


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
part2()
