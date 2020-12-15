mod aoc2020;

use std::time::Instant;
use structopt::StructOpt;

/// Run a solution for a given year and day
#[derive(StructOpt, Debug)]
#[structopt(name = "AdventOfCode")]
struct Opt {
    /// Day selection for task
    #[structopt(short, long)]
    day: Option<usize>,
    /// Pass data to use instead of input/DAY.in
    #[structopt(long)]
    data: Option<String>,
}

type SolutionFunc = fn() -> anyhow::Result<()>;

fn main() {
    println!("Advent of Code");
    println!("==============");
    let args = Opt::from_args();

    let solutions: Vec<(usize, SolutionFunc)> = vec![
        (1, aoc2020::day01::day01),
        (2, aoc2020::day02::day02),
        (3, aoc2020::day03::day03),
        (4, aoc2020::day04::day04),
        (5, aoc2020::day05::day05),
        (6, aoc2020::day06::day06),
        (7, aoc2020::day07::day07),
        (8, aoc2020::day08::day08),
        (9, aoc2020::day09::day09),
        (10, aoc2020::day10::day10),
        (11, aoc2020::day11::day11),
        (12, aoc2020::day12::day12),
        (13, aoc2020::day13::day13),
        (14, aoc2020::day14::day14),
    ];

    let start = Instant::now();
    for (day, function) in solutions {
        if let Some(d) = args.day {
            if day != d {
                continue;
            }
        }
        run_and_time_solution(day, function);
    }
    println!("Time for all: {}ms", as_ms(start.elapsed()));
}

fn as_ms(t: std::time::Duration) -> f64 {
    t.as_nanos() as f64 / 1E6
}

fn run_and_time_solution(day: usize, function: SolutionFunc) {
    let now = Instant::now();
    match function() {
        Ok(_) => println!("    Time: {:.2}ms", as_ms(now.elapsed())),
        Err(e) => eprintln!("D{}: {}", day, e),
    }
}
