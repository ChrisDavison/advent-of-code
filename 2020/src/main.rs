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
mod day11;
mod day12;
mod day13;
mod day14;

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
        (11, day11::day11),
        (12, day12::day12),
        (13, day13::day13),
        (14, day14::day14),
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
            let filename: String = match args.data.clone() {
                Some(filename) => filename,
                None => format!("input/{:02}.in", solution.0),
            };
            let data = std::fs::read_to_string(&filename)
                .unwrap_or_else(|_| panic!("Data file {} doesn't exist", filename));
            match solution.1(&data) {
                Ok(_) => println!("    Time: {:.2}ms", (now.elapsed().as_nanos() as f64) / 1E6),
                Err(e) => eprintln!("D{}: {}", solution.0, e),
            }
        }
    }
    println!(
        "Time for all: {}ms",
        (start.elapsed().as_nanos() as f64) / 1E6
    );
}
