#![allow(dead_code, unused_variables)]
mod days;
mod part;

use days::*;

use anyhow::{Context, Result};
use part::Part;

fn main() {
    let day = std::env::args().skip(1).nth(0).and_then(|x| x.parse().ok());
    let funcs: Vec<fn(&[&str], Part) -> Result<()>> = vec![
        day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13, day14,
        day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25,
    ];

    println!("Advent of Code 2020");
    println!("");

    for (i, func) in funcs.iter().enumerate() {
        if let Some(d) = day {
            if (i + 1) != d {
                continue;
            }
        }
        if let Err(e) = run(i + 1, *func) {
            eprintln!("{}", e);
        }
        if day.is_none() {
            println!("");
        }
    }
}

fn run(day: usize, func: fn(&[&str], Part) -> Result<()>) -> Result<()> {
    let data = std::fs::read_to_string(format!("input/day{}.txt", day))
        .with_context(|| format!("No input file for day {}. Assuming day not released.", day))?;
    let tidy_data: Vec<&str> = data.split("\n").map(|x| x.trim()).collect();
    if let Err(e) = func(&tidy_data, Part::One) {
        eprintln!("ERR {}.{} - {}", day, Part::One, e);
    }
    if let Err(e) = func(&tidy_data, Part::Two) {
        eprintln!("ERR {}.{} - {}", day, Part::Two, e);
    }
    Ok(())
}
