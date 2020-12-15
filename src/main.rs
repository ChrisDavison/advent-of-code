mod aoc2020;
mod time_macro;

const SOLUTIONS: &[fn() -> anyhow::Result<()>] = &[
    aoc2020::day01,
    aoc2020::day02,
    aoc2020::day03,
    aoc2020::day04,
    aoc2020::day05,
    aoc2020::day06,
    aoc2020::day07,
    aoc2020::day08,
    aoc2020::day09,
    aoc2020::day10,
    aoc2020::day11,
    aoc2020::day12,
    aoc2020::day13,
    aoc2020::day14,
    aoc2020::day15,
];

fn main() {
    let day_wanted: Option<usize> = std::env::args()
        .skip(1)
        .next()
        .map(|x| x.parse::<usize>().ok())
        .unwrap_or(None);

    println!("Advent of Code");
    println!("==============");
    for (day, function) in SOLUTIONS.iter().enumerate() {
        if let Some(d) = day_wanted {
            if day != d {
                continue;
            }
        }
        time_solution!(day, function());
    }
}
