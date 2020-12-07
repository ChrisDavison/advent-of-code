mod year2020;
use anyhow::anyhow;

fn main() {
    println!("Advent of Code");
    let args: Vec<String> = std::env::args().skip(1).collect();
    let year: u64 = args
        .get(0)
        .and_then(|x| Some(x.parse::<u64>().ok()?))
        .unwrap_or(0);
    let day: u64 = args
        .get(1)
        .and_then(|x| Some(x.parse::<u64>().ok()?))
        .unwrap_or(0);

    let result = match (year, day) {
        (2020, 1) => year2020::day01::solve(),
        (2020, 2) => year2020::day02::solve(),
        (2020, 3) => year2020::day03::solve(),
        (2020, 4) => year2020::day04::solve(),
        (2020, 5) => year2020::day05::solve(),
        (2020, 6) => year2020::day06::solve(),
        (2020, 7) => year2020::day07::solve(),
        _ => Err(anyhow!("Day hasn't been done")),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}
