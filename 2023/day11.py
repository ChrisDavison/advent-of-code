from utility import reduce, Path, timed, dataclass, defaultdict, m, re, char_indices

SAMPLE = """...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."""

DATA = Path("input/11").read_text()


@timed
def part1(data=SAMPLE):
    return run(data, add_rows=1)


def run(data=SAMPLE, add_rows=1):
    gdict = parser(data, add_rows=add_rows)
    dists = dict()
    dists_per = defaultdict(list)
    for start, ends in gdict.items():
        for end in ends:
            if (start, end) in dists or (end, start) in dists:
                continue
            dist = start.dist(end)
            dists[(start, end)] = dist
            dists_per[start].append(dist)
    # pp(dists)
    mins = dists.values()
    # pp(sum(mins))
    # pp(dists_per)
    return(sum(mins))


@timed
def part2(data=SAMPLE):
    # in part1, we add a line for each blank
    # instead, in part2 we _replace_ with 1_000_000 lines
    # (i.e. add 999_999)
    return run(data, add_rows=1_000_000-1)


@dataclass
class Galaxy:
    x: int
    y: int
    n: int

    def __eq__(self, o):
        return self.x == o.x and self.y == o.y

    def __hash__(self):
        return (self.x, self.y).__hash__()

    def expand(self, blankrows, blankcols, add_rows=1):
        dy = sum(add_rows for b in blankrows if b < self.y)
        dx = sum(add_rows for b in blankcols if b < self.x)
        self.x += dx
        self.y += dy

    def __repr__(self):
        return f"#{self.n}: {self.x},{self.y}"

    def dist(self, o):
        return int(m.fabs(self.x - o.x) + m.fabs(self.y - o.y))


def parse_galaxies(data):
    galaxies = []
    blank_rows = []
    blank_cols = set(range(len(data.splitlines()[0])))
    n = 0
    for y, line in enumerate(data.splitlines()):
        found_galaxy = False
        for x in char_indices(line, '#'):
            n += 1
            galaxies.append(Galaxy(x, y, n))
            found_galaxy = True
        if not found_galaxy:
            blank_rows.append(y)
        blank_cols &= char_indices(line, '.')
    # from blanks _per_row_, find which were blank in _all_ rows
    # blank_cols = reduce(lambda x, y: x.intersection(y), blank_cols)
    # now we have the galaxies, blank rows, and blank columns,
    # expand the galaxies' position based on the blanks
    return galaxies, blank_rows, blank_cols


def parser(data, add_rows=1):
    galaxies, blank_rows, blank_cols = parse_galaxies(data)
    # from blanks _per_row_, find which were blank in _all_ rows
    # blank_cols = reduce(lambda x, y: x.intersection(y), blank_cols)
    # now we have the galaxies, blank rows, and blank columns,
    # expand the galaxies' position based on the blanks
    for g in galaxies:
        g.expand(blank_rows, blank_cols, add_rows=add_rows)

    return {x: [y for y in galaxies if x != y] for x in galaxies}


part1(DATA)
part2(DATA)
