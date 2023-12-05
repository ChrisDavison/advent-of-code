use aoc2023::*;

#[allow(dead_code)]
const SAMPLE: &str = r"seeds: 79 14 55 13

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
56 93 4";

const DATA: &str = include_str!("../input/day05");

const USE_SAMPLE: bool = false;

fn main() {
    let parsed = parse(if USE_SAMPLE { SAMPLE } else { DATA });

    timed! {1, part1, parsed};
    timed! {1, part2, parsed};
}

fn parse(data: &str) -> (Vec<usize>, Vec<Cropmap>) {
    let mut paragraphs = data.split("\n\n");
    let seeds = numbers(paragraphs.next().unwrap().split_once(':').unwrap().1);

    let paragraphs = paragraphs
        .map(|paragraph| {
            paragraph
                .lines()
                .skip(1)
                .map(parse_map)
                .collect::<Cropmap>()
        })
        .collect::<Vec<Cropmap>>();

    (seeds, paragraphs)
}

fn parse_map(line: &str) -> Croprange {
    let nums = numbers(line);
    (nums[0], nums[1], nums[1] + nums[2])
}

fn min_location_from_seedset(seeds: &mut [usize], maps: &[Cropmap]) -> usize {
    for (i, map) in maps.iter().enumerate() {
        get_next_value(seeds, map);
    }
    *seeds.iter().min().unwrap()
}

type Croprange = (usize, usize, usize);
type Cropmap = Vec<Croprange>;

fn get_next_value(seeds: &mut [usize], ranges: &Cropmap) {
    for seed in seeds.iter_mut() {
        for (out_start, in_min, in_max) in ranges {
            if *seed >= *in_min && *seed <= *in_max {
                *seed = out_start + (*seed - in_min);
                break;
            }
        }
    }
}

fn get_next_ranges(seedrange: (usize, usize), cropmaps: &Cropmap) -> Vec<(usize, usize)> {
    let mut seedrange = seedrange;
    let mut ranges = Vec::new();
    let mut i = 0;
    loop {
        let (to, from, delta) = cropmaps[i];
        // If seeds start BEFORE the minimum crop range
        // then ranges start from seed range
        if seedrange.0 < from {
            if seedrange.1 < to {
                return vec![seedrange];
            }
            // We don't cover the full range, so set the range that the seed covers with this
            // range, then update the seed to be the part that WASN'T covered
            ranges.push((seedrange.0, from - 1));
            seedrange.0 = from;
        } else if from <= seedrange.0 && seedrange.0 < (from + delta) {
            // The seed starts WITHIN a cropmap range
            let seeddelta = to - from;
            if seedrange.1 < (from + seeddelta) {
                // ...and ends within the cropmap range
                return vec![(seedrange.0 + seeddelta, seedrange.1 + seeddelta)];
            }
            // we end AFTER cropmap range,
            // so push the initial chunk to the range
            ranges.push((seedrange.0 + seeddelta, to + seeddelta));
            // ...and keep the seed bit after the cropmap
            seedrange.0 = from + delta
        } else if from + delta < seedrange.0 {
            // this crop map ends before the seed,
            // so bump i to continue
            i += 1;
            if i == cropmaps.len() {
                break;
            }
        }
    }
    ranges.push(seedrange);
    ranges
}

fn part1(parsed: &(Vec<usize>, Vec<Cropmap>)) -> Result<usize> {
    let (seeds, maps) = parsed;
    let mut seeds = seeds.clone();
    Ok(min_location_from_seedset(&mut seeds, &maps))
}

fn part2(parsed: &(Vec<usize>, Vec<Cropmap>)) -> Result<usize> {
    let (seeds, maps) = parsed;

    let mut seed_startends = pairs(seeds)
        .map(|ab| (ab.0, ab.0 + ab.1))
        .collect::<Vec<_>>();

    // let mut out = Vec::new();
    for (i, map) in maps.iter().enumerate() {
        println!("{:#?}", i);
        let mut newranges = Vec::new();
        for rng in &seed_startends {
            for nrng in get_next_ranges(*rng, map) {
                newranges.push(nrng);
            }
        }
        seed_startends = newranges;
        println!("{:#?}", seed_startends);
    }
    // for (i, seed_startend) in seed_startends.iter().enumerate() {
    //     out.push(min_location_from_seedset_range(*seed_startend, &maps));
    // }
    // Ok(*out.iter().min().unwrap_or(&0))
    Ok(0)
}
