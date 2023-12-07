import math as m
from pathlib import Path
from utility import *
from pprint import pprint as pp

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

DATA = Path('input/05').read_text()


# --- part 1
def get_next_value(seeds, ranges):
    print(f"{seeds = :}")
    for i in range(0, len(seeds)):
        print(f"{ranges[0] = :}")
        for out_start, in_min, in_max in ranges:
            if in_min <= seeds[i] <= in_max:
                seeds[i] = out_start + (seeds[i] - in_min)
                break
    return seeds


def min_location_from_seedset(seeds, maps):
    for _, map in enumerate(maps):
        seeds = get_next_value(seeds, map)
    return min(seeds)


def parsed(data):
    pp = paragraphs(data)
    seeds = mapl(int, re.findall(r"\d+", pp[0]))

    maps = []
    for paragraph in pp[1:]:
        to_from_deltas = []
        for line in paragraph.splitlines()[1:]:
            lnums = mapl(int, re.findall(r"\d+", line))
            to_from_deltas.append((lnums[0], lnums[1], lnums[1] + lnums[2]))
        maps.append(to_from_deltas)
    return seeds, maps


def f(data):
    seeds, maps = parsed(data)
    return min_location_from_seedset(seeds, maps)


print(f(SAMPLE))
# assert f(SAMPLE) == ...
# assert f(DATA) == ...


def get_next_ranges(cropmapping, seed_range):
    ranges, i = [], 0
    while True:
        to_from_delta = cropmapping[i]
        if to_from_delta[1] > seed_range[0]:
            #   start ... cropstart ... end
            # e.g., for this seed, we either stay the samr or get converted
            # within the mapping boundaries
            if to_from_delta[1] > seed_range[1]:
                ranges.append(seed_range)
                return ranges
            # else... range += seedstart ..cropstart-1
            ranges.append((seed_range[0], to_from_delta[1] - 1))
            # seedstart = cropstart..seedend
            seed_range = to_from_delta[1], seed_range[1]

        elif to_from_delta[1] <= seed_range[0] < to_from_delta[1] + to_from_delta[2]:
            offset = to_from_delta[0] - to_from_delta[1]
            # cropstart <= start ... cropend
            if seed_range[1] < to_from_delta[1] + to_from_delta[2]:
                ranges.append(tuple(r + offset for r in seed_range))
                return ranges
            ranges.append((seed_range[0] + offset, to_from_delta[0] + to_from_delta[2]))
            seed_range = to_from_delta[1] + to_from_delta[2], seed_range[1]
        elif to_from_delta[1] + to_from_delta[2] <= seed_range[0]:
            i += 1
            if i == len(cropmapping):
                break
    ranges.append(seed_range)
    return ranges


# --- part 2
def f2(data):
    seeds, maps = parsed(data)
    seedranges = [(seeds[i], seeds[i] + seeds[i + 1]) for i in range(0, len(seeds), 2)]
    for c in maps[1:]:
        print(c)
        # c = [to from delta]...

        # sort for part 2
        print(f"{c = :}")
        sorted_cropmap = sorted(
            [c[k : k + 3] for k in range(0, len(c), 3)], key=lambda x: x[1]
        )
        print(f"{sorted_cropmap[0] = :}")
        seeds = get_next_value(seeds, sorted_cropmap)
        print(f"{seeds = :}")
        gen = (get_next_ranges(sorted_cropmap, rng) for rng in seedranges)
        print(f"{seeds[0] = :}")
        ranges = sum(gen, [])
        print()
        break


f2(SAMPLE)
# assert f2(SAMPLE) == ...
# assert f2(DATA) == ...
