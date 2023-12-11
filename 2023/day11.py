from utility import reduce, Path, timed, dataclass, defaultdict, m

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
    return run(data, add_rows=2)


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
    return run(data, add_rows=1_000_000)



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
        dy = sum(add_rows-1 for b in blankrows if b < self.y)
        dx = sum(add_rows-1 for b in blankcols if b < self.x)
        self.x += dx
        self.y += dy

    def __repr__(self):
        return f"#{self.n}: {self.x},{self.y}"

    def dist(self, o):
        return int(m.fabs(self.x - o.x) + m.fabs(self.y - o.y))


def parser(data, add_rows=1):
    galaxies = []
    blank_rows = []
    blank_cols = []
    n = 0
    for y, line in enumerate(data.splitlines()):
        found_galaxy = False
        for x, char in enumerate(line):
            if char == '#':
                n += 1
                galaxies.append(Galaxy(x, y, n))
                found_galaxy = True
        blank_cols.append(set(i for i in range(len(line)) if line[i] == '.'))
        if not found_galaxy:
            blank_rows.append(y)
    # from blanks _per_row_, find which were blank in _all_ rows
    blank_cols = reduce(lambda x, y: x.intersection(y), blank_cols)
    # now we have the galaxies, blank rows, and blank columns,
    # expand the galaxies' position based on the blanks
    for g in galaxies:
        g.expand(blank_rows, blank_cols, add_rows=add_rows)

    return {x: [y for y in galaxies if x != y] for x in galaxies}


part1(DATA)
part2(DATA)
