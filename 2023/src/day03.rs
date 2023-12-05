use aoc2023::*;

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

fn main() {
    timed! {3, part1, DATA};
    timed! {3, part2, DATA};
}

fn points_surrounding_region(region: &[Point2D]) -> impl Iterator<Item = Point2D> + '_ {
    region.iter().flat_map(|point| point.surrounding())
}

fn part1(data: &str) -> Result<usize> {
    let (number_map, symbol_locations) = parse(data);

    let mut parts = Vec::new();
    for (number, region) in number_map {
        if points_surrounding_region(&region).any(|s| symbol_locations.contains_key(&s)) {
            parts.push(number);
        }
    }

    Ok(parts.iter().sum())
}

fn part2(data: &str) -> Result<usize> {
    let (number_map, symbol_locations) = parse(data);

    let mut point_to_idx_of_partnum = HashMap::new();
    let mut partnums = Vec::new();

    for (number, region) in &number_map {
        partnums.push(number);
        for p in region {
            point_to_idx_of_partnum.insert(p, partnums.len() - 1);
        }
    }

    let mut ratios = Vec::new();
    for (location, ch) in symbol_locations {
        if ch != '*' {
            continue;
        }
        let mut candidates = HashSet::new();
        for p in points_surrounding_region(&[location]) {
            if let Some(partnum_idx) = point_to_idx_of_partnum.get(&p) {
                candidates.insert(partnums[*partnum_idx]);
            }
        }
        if candidates.len() == 2 {
            ratios.push(candidates.iter().copied().product::<usize>());
        }
    }

    Ok(ratios.iter().sum())
}

type SymbolLocations = BTreeMap<Point2D, char>;
type Region = Vec<Point2D>;

fn parse(data: &str) -> (Vec<(usize, Region)>, SymbolLocations) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?<digit>\d+)|(?<sym>[^0-9.])").expect("Failed to crate regex");
    }

    let mut possible = Vec::new();
    let mut symbolmap = BTreeMap::new();

    for (row, line) in data.lines().enumerate() {
        for m in RE.captures_iter(line) {
            if let Some(m) = m.name("sym") {
                symbolmap.insert(
                    Point2D {
                        col: m.start() as isize,
                        row: row as isize,
                    },
                    m.as_str().chars().next().unwrap(),
                );
            }
            if let Some(m) = m.name("digit") {
                let number_locations = (m.start()..m.end())
                    .map(|col| Point2D {
                        row: row as isize,
                        col: col as isize,
                    })
                    .collect();
                possible.push((m.as_str().parse().unwrap(), number_locations));
            }
        }
    }
    (possible, symbolmap)
}
