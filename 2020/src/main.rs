#![allow(dead_code, unused_variables)]
mod day1;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
mod day2;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;
mod day3;
mod day4;
mod day5;
// mod day6;
// mod day7;
// mod day8;
// mod day9;
mod part;

use anyhow::Result;
use part::Part;

fn main() {
    let day = std::env::args()
        .skip(1)
        .nth(0)
        .unwrap_or("0".to_string())
        .parse()
        .unwrap();
    let funcs: Vec<fn(&[&str], Part) -> Result<()>> =
        vec![day1::day1, day2::day2, day3::day3, day4::day4, day5::day5];

    println!("Advent of Code 2020");
    println!("");

    for (i, func) in funcs.iter().enumerate() {
        if (i + 1) == day || day == 0 {
            let data = match std::fs::read_to_string(format!("input/day{}.txt", i + 1)) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("No input file for day {}", i + 1);
                    eprintln!("Assuming that's the end of days that've been completed");
                    continue;
                }
            };
            let tidy_data: Vec<&str> = data.split("\n").map(|x| x.trim()).collect();
            if let Err(e) = func(&tidy_data, Part::One) {
                eprintln!("ERR {}.{} - {}", i + 1, Part::One, e);
            }
            if let Err(e) = func(&tidy_data, Part::Two) {
                eprintln!("ERR {}.{} - {}", i + 1, Part::Two, e);
            }
            if day == 0 && i != funcs.len() - 1 {
                println!("")
            }
        }
    }
}
