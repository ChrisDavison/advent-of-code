use aoc2023::*;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)").expect("Failed to crate regex");
}

#[allow(dead_code)]
const SAMPLE: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

const DATA: &str = include_str!("../input/day03");

pub fn day03() -> Result<String> {
    Ok(format!(
        "2023 03.1 -> {}\n2023 03.2 -> {}",
        part1(DATA)?,
        part2(DATA)?
    ))
}

fn points_surrounding_region(region: &[Point2D]) -> BTreeSet<Point2D> {
    let mut out = BTreeSet::new();
    for point in region {
        for surround in point.surrounding() {
            out.insert(surround);
        }
    }
    out
}

fn part1(data: &str) -> Result<usize> {
    let (number_map, symbol_locations) = parse(data);

    let symbol_keys = &symbol_locations.keys().cloned().collect::<BTreeSet<_>>();
    let mut parts = Vec::new();
    for (number, region) in number_map {
        let surround = points_surrounding_region(&region);
        let both: BTreeSet<_> = surround.intersection(symbol_keys).collect();
        if !both.is_empty() {
            parts.push(number);
        }
    }

    Ok(parts.iter().sum())
}

fn part2(data: &str) -> Result<usize> {
    let (number_map, symbol_locations) = parse(data);

    let mut ratios = Vec::new();
    for (location, ch) in symbol_locations {
        if ch != '*' {
            continue;
        }
        let mut candidates = Vec::new();
        let mut regions_seen = HashSet::new();
        for p in points_surrounding_region(&[location]) {
            for (number, region) in &number_map {
                if region.contains(&p) && !regions_seen.contains(region) {
                    candidates.push(number);
                    regions_seen.insert(region);
                }
            }
            // find any number that has p in it's region
        }
        if candidates.len() == 2 {
            ratios.push(candidates[0] * candidates[1]);
        }
    }

    Ok(ratios.iter().sum())
}

fn is_symbol(ch: char) -> bool {
    !"0123456789.".contains(ch)
}

fn parse(data: &str) -> (Vec<(usize, Vec<Point2D>)>, BTreeMap<Point2D, char>) {
    let mut possible = Vec::new();
    let mut symbolmap = BTreeMap::new();

    for (row, line) in data.lines().enumerate() {
        for m in RE.find_iter(line) {
            let number_locations = (m.start()..m.end())
                .map(|col| Point2D {
                    row: row as isize,
                    col: col as isize,
                })
                .collect();
            possible.push((m.as_str().parse().unwrap(), number_locations));
        }
        for (col, c) in line.chars().enumerate() {
            if is_symbol(c) {
                symbolmap.insert(
                    Point2D {
                        col: col as isize,
                        row: row as isize,
                    },
                    c,
                );
            }
        }
    }
    (possible, symbolmap)
}
