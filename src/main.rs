mod year2020;
use anyhow::anyhow;
use structopt::StructOpt;

/// Run a solution for a given year and day
#[derive(StructOpt, Debug)]
#[structopt(name = "AdventOfCode")]
struct Opt {
    /// Year selection for task
    year: u64,
    /// Day selection for task
    day: u64,
}

fn main() {
    println!("Advent of Code");
    let args = Opt::from_args();

    let result = match (args.year, args.day) {
        (2020, 1) => year2020::day01::solve(),
        (2020, 2) => year2020::day02::solve(),
        (2020, 3) => year2020::day03::solve(),
        (2020, 4) => year2020::day04::solve(),
        (2020, 5) => year2020::day05::solve(),
        (2020, 6) => year2020::day06::solve(),
        (2020, 7) => year2020::day07::solve(),
        (2020, 8) => year2020::day08::solve(),
        _ => Err(anyhow!("Day hasn't been done")),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}
