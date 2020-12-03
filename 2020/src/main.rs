mod day1;
mod day2;
mod day3;
mod util;

#[derive(Clone, Copy)]
pub enum Part {
    One,
    Two,
}

type DayFunction = fn(Part) -> anyhow::Result<String>;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let day = if args.len() > 0 {
        args[0].parse().unwrap()
    } else {
        0
    };

    let functions: Vec<DayFunction> = vec![day1::run, day2::run, day3::run];

    if day == 0 {
        eprintln!("Using days numbered from 1");
        std::process::exit(1);
    }
    if day > functions.len() {
        eprintln!("Not completed day {} yet", day);
        std::process::exit(2);
    }
    for i in 0..functions.len() {
        let func = functions[i];
        let counter = i + 1;
        if counter == day {
            match func(Part::One) {
                Ok(result) => println!("Day {} Part 1: {}", counter, result),
                Err(e) => eprintln!("ERROR Day {} Part 1: {}", counter, e),
            }
            match func(Part::Two) {
                Ok(result) => println!("Day {} Part 2: {}", counter, result),
                Err(e) => eprintln!("ERROR Day {} Part 2: {}", counter, e),
            }
        }
    }
}
