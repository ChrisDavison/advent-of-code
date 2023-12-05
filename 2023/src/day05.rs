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

pub fn day05() -> Result<(usize, usize)> {
    let parsed = if USE_SAMPLE { SAMPLE } else { DATA };
    Ok((part1(&parsed), part2(&parsed)))
}

fn part1(data: &str) -> usize {
    // let mut mapmap = HashMap::new();
    let mut paragraphs = data.split("\n\n");
    let mut seeds = numbers(paragraphs.next().unwrap().split_once(':').unwrap().1);
    for paragraph in paragraphs {
        let mut plines = paragraph.lines();

        // skip the first line (a-to-b map)
        let _words = plines.next().unwrap().split_once(' ').unwrap().0.split('-');

        let mut converted = HashSet::new();
        let mut seeds_remaining = seeds.iter().cloned().collect::<HashSet<usize>>();
        let mut seeds_new = HashSet::new();

        for line in plines {
            let num = numbers(line);
            let from_min = num[1];
            let from_max = num[1] + num[2];
            let to_min = num[0];
            for seed in &seeds {
                if seeds_remaining.contains(&seed) {
                    if *seed >= from_min && *seed <= from_max {
                        // println!("conv {} to {}", seed, to_min + (seed - from_min));
                        converted.insert(seed);
                        seeds_new.insert(to_min + (seed - from_min));
                        seeds_remaining.remove(&seed);
                    }
                }
            }
        }
        seeds_new.extend(seeds_remaining);
        seeds = seeds_new.iter().cloned().collect();
    }
    *seeds.iter().min().unwrap()
}

fn part2(data: &str) -> usize {
    // let mut mapmap = HashMap::new();
    let mut paragraphs = data.split("\n\n");
    let seedline = paragraphs.next().unwrap().split_once(':').unwrap().1;
    // let mut seeds = numbers();
    println!("{:#?}", number_pairs(seedline));
    // for paragraph in paragraphs {
    //     let mut plines = paragraph.lines();

    //     // skip the first line (a-to-b map)
    //     let _words = plines.next().unwrap().split_once(' ').unwrap().0.split('-');

    //     let mut converted = HashSet::new();
    //     let mut seeds_remaining = seeds.iter().cloned().collect::<HashSet<usize>>();
    //     let mut seeds_new = HashSet::new();

    //     for line in plines {
    //         let num = numbers(line);
    //         let from_min = num[1];
    //         let from_max = num[1] + num[2];
    //         let to_min = num[0];
    //         for seed in &seeds {
    //             if seeds_remaining.contains(&seed) {
    //                 if *seed >= from_min && *seed <= from_max {
    //                     // println!("conv {} to {}", seed, to_min + (seed - from_min));
    //                     converted.insert(seed);
    //                     seeds_new.insert(to_min + (seed - from_min));
    //                     seeds_remaining.remove(&seed);
    //                 }
    //             }
    //         }
    //     }
    //     seeds_new.extend(seeds_remaining);
    //     seeds = seeds_new.iter().cloned().collect();
    // }
    // *seeds.iter().min().unwrap()
    0
}
