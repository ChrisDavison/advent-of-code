mod year2020;
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
}

type Solution = (u64, u64, fn() -> anyhow::Result<()>);

fn main() {
    println!("Advent of Code");
    let args = Opt::from_args();

    let solutions: Vec<Solution> = vec![
        (2020, 1, year2020::day01::solve),
        (2020, 2, year2020::day02::solve),
        (2020, 3, year2020::day03::solve),
        (2020, 4, year2020::day04::solve),
        (2020, 5, year2020::day05::solve),
        (2020, 6, year2020::day06::solve),
        (2020, 7, year2020::day07::solve),
        (2020, 8, year2020::day08::solve),
    ];

    for solution in solutions {
        let run = match (args.year, args.day) {
            (Some(y), Some(d)) => solution.0 == y && solution.1 == d,
            (Some(y), None) => solution.0 == y,
            (None, Some(d)) => solution.1 == d,
            (None, None) => true,
        };
        if run {
            if let Err(e) = solution.2() {
                eprintln!("{}", e);
            }
        }
    }
}
