mod year2020;
use std::time::Instant;
use structopt::StructOpt;

/// Run a solution for a given year and day
#[derive(StructOpt, Debug)]
#[structopt(name = "AdventOfCode")]
struct Opt {
    /// Year selection for task
    #[structopt(short, long)]
    year: Option<u64>,
    /// Day selection for task
    #[structopt(short, long)]
    day: Option<u64>,
    /// Run using the sample input
    #[structopt(short, long)]
    sample: bool,
}

type Solution = (u64, u64, fn(&str) -> anyhow::Result<()>);

fn main() {
    println!("Advent of Code");
    let args = Opt::from_args();

    let solutions: Vec<Solution> = vec![
        (2020, 1, year2020::day01),
        (2020, 2, year2020::day02),
        (2020, 3, year2020::day03),
        (2020, 4, year2020::day04),
        (2020, 5, year2020::day05),
        (2020, 6, year2020::day06),
        (2020, 7, year2020::day07),
        (2020, 8, year2020::day08),
        (2020, 9, year2020::day09),
        (2020, 10, year2020::day10),
    ];

    for solution in solutions {
        let run = match (args.year, args.day) {
            (Some(y), Some(d)) => solution.0 == y && solution.1 == d,
            (Some(y), None) => {
                println!("");
                solution.0 == y
            }
            (None, Some(d)) => solution.1 == d,
            (None, None) => {
                println!("");
                true
            }
        };
        if run {
            let now = Instant::now();
            let data = if args.sample {
                std::fs::read_to_string(format!("input/sample{}.in", solution.1))
                    .expect("Day doesn't exist")
            } else {
                std::fs::read_to_string(format!("input/day{}.in", solution.1))
                    .expect("Day doesn't exist")
            };
            let result = solution.2(&data);
            if let Err(e) = result {
                eprintln!("{}", e);
            } else {
                println!("    Time: {}us", now.elapsed().as_micros());
            }
        }
    }
}
