from utility import re, Path, timed, directions8, np

SAMPLE = """467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."""

DATA = Path("input/03").read_text()


def p(data):
    possible = []
    symbols = dict()
    rxline = re.compile(r"(?P<digit>\d+)|(?P<sym>[^0-9.])")
    for i, line in enumerate(data.splitlines()):
        for m in rxline.finditer(line):
            if m.group("sym"):
                symbols[(i, m.start())] = m.group("sym")
            if m.group("digit"):
                possible.append(
                    (
                        int(m.group("digit")),
                        [(i, col) for col in range(m.start(), m.end())],
                    )
                )
    return possible, symbols


def surrounding(point):
    point = tuple(point)
    for direction in directions8:
        yield point[0] + direction[0], point[1] + direction[1]


def points_surrounding_region(r):
    seen = set()
    for point in r:
        for surround in surrounding(point):
            if surround not in seen:
                seen.add(surround)
                yield surround


@timed
def part1():
    parts = []
    for n, region in number_map:
        if any(p in symbols for p in points_surrounding_region(region)):
            parts.append(n)
    print(sum(parts))


number_map, symbols = p(DATA)
part1()


@timed
def part2():
    partnums = []
    point_to_idx_of_partnum = dict()
    for number, region in number_map:
        partnums.append(number)
        for point in region:
            point_to_idx_of_partnum[point] = len(partnums) - 1

    ratios = []
    for location, ch in symbols.items():
        if ch != "*":
            continue
        candidates = set()
        for p in points_surrounding_region([location]):
            if idx := point_to_idx_of_partnum.get(p, None):
                candidates.add(partnums[idx])
        if len(candidates) == 2:
            ratios.append(np.prod(list(candidates)))
    print(sum(ratios))


part2()
