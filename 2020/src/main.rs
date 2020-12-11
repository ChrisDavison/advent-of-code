mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

use std::time::Instant;
use structopt::StructOpt;

/// Run a solution for a given year and day
#[derive(StructOpt, Debug)]
#[structopt(name = "AdventOfCode")]
struct Opt {
    /// Day selection for task
    #[structopt(short, long)]
    day: Option<usize>,
    /// Run using the sample input
    #[structopt(short, long)]
    sample: bool,
}

type SolutionFunc = fn(&str) -> anyhow::Result<()>;

fn main() {
    println!("Advent of Code");
    let args = Opt::from_args();

    let solutions: Vec<(usize, SolutionFunc)> = vec![
        (1, day01::day01),
        (2, day02::day02),
        (3, day03::day03),
        (4, day04::day04),
        (5, day05::day05),
        (6, day06::day06),
        (7, day07::day07),
        (8, day08::day08),
        (9, day09::day09),
        (10, day10::day10),
    ];

    let start = Instant::now();
    for solution in solutions {
        let run = match args.day {
            Some(d) => solution.0 == d,
            None => {
                println!();
                true
            }
        };
        if run {
            let now = Instant::now();
            let data = if args.sample {
                std::fs::read_to_string(format!("input/sample{}.in", solution.0))
                    .expect("Day doesn't exist")
            } else {
                std::fs::read_to_string(format!("input/day{}.in", solution.0))
                    .expect("Day doesn't exist")
            };
            let result = solution.1(&data);
            if let Err(e) = result {
                eprintln!("{}", e);
            } else {
                println!("    Time: {:.2}ms", (now.elapsed().as_nanos() as f64) / 1E6);
            }
        }
    }
    println!(
        "Time for all: {}ms",
        (start.elapsed().as_nanos() as f64) / 1E6
    );
}
