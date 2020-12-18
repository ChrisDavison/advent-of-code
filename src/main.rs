mod aoc2020;
mod time_macro;
extern crate lazy_static;

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
    aoc2020::day16,
    aoc2020::day17,
    aoc2020::day18,
];

fn main() {
    let day_wanted: Option<usize> = std::env::args()
        .nth(1)
        .map(|x| x.parse::<usize>().ok())
        .unwrap_or(None);

    for (day, function) in SOLUTIONS.iter().enumerate() {
        if let Some(d) = day_wanted {
            if day != d - 1 {
                continue;
            }
        }
        time_solution!(day, function());
    }
}
