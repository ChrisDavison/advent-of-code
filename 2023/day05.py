from utility import re, paragraphs, mapl, timed, Path, ints

SAMPLE = """seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"""

DATA = Path("input/05").read_text()


def parsed(data):
    seeds, *pp = paragraphs(data)
    seeds = ints(seeds)

    maps = [[ints(line) for line in paragraph.splitlines()[1:]] for paragraph in pp[1:]]
    return seeds, maps


@timed
def part1(data):
    seeds, maps = parsed(data)
    for rng in maps:
        new = []
        for s in seeds:
            for to, frm, delta in rng:
                if frm <= s < frm + delta:
                    new.append(s - frm + to)
                    break
            else:
                new.append(s)
        seeds = new
    return min(seeds)


@timed
def part2(data):
    seeds, maps = parsed(data)
    seedranges = [(seeds[i], seeds[i] + seeds[i + 1]) for i in range(0, len(seeds), 2)]
    for rng in maps:
        new = []
        while len(seedranges) > 0:
            s, e = seedranges.pop()
            for to, frm, delta in rng:
                os = max(s, frm)
                oe = min(e, frm + delta)
                if os < oe:
                    new.append((os - frm + to, oe - frm + to))
                    if os > s:
                        seedranges.append((s, os))
                    if e > oe:
                        seedranges.append((oe, e))
                    break
            else:
                new.append((s, e))
        seedranges = new
    return min(seedranges)[0]


part1(DATA)
part2(DATA)
