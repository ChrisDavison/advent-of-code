#![allow(dead_code, unused_variables)]
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
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod part;

use anyhow::Result;
use part::Part;

fn main() {
    let day = std::env::args().skip(1).nth(0).and_then(|x| x.parse().ok());
    let funcs: Vec<fn(&[&str], Part) -> Result<()>> = vec![
        day01::day1,
        day02::day2,
        day03::day3,
        day04::day4,
        day05::day5,
        day06::day6,
        day07::day7,
        day08::day8,
        day09::day9,
        day10::day10,
        day11::day11,
        day12::day12,
        day13::day13,
        day14::day14,
        day15::day15,
        day16::day16,
        day17::day17,
        day18::day18,
        day19::day19,
        day20::day20,
        day21::day21,
        day22::day22,
        day23::day23,
        day24::day24,
        day25::day25,
    ];

    println!("Advent of Code 2020");
    println!("");

    for (i, func) in funcs.iter().enumerate() {
        if let Some(d) = day {
            if (i + 1) != d {
                continue;
            }
        }
        let data = match std::fs::read_to_string(format!("input/day{}.txt", i + 1)) {
            Ok(d) => d,
            Err(e) => {
                eprintln!(
                    "No input file for day {}. Assuming day not released.",
                    i + 1
                );
                break;
            }
        };
        let tidy_data: Vec<&str> = data.split("\n").map(|x| x.trim()).collect();
        if let Err(e) = func(&tidy_data, Part::One) {
            eprintln!("ERR {}.{} - {}", i + 1, Part::One, e);
        }
        if let Err(e) = func(&tidy_data, Part::Two) {
            eprintln!("ERR {}.{} - {}", i + 1, Part::Two, e);
        }
        if day.is_none() {
            println!("");
        }
    }
}
