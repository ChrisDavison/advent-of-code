#![allow(dead_code, unused_variables)]
mod aoc2020;
mod part;

use aoc2020::*;

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
            break;
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
    func(&tidy_data, Part::One).with_context(|| format!("ERR {}.1", day))?;
    func(&tidy_data, Part::Two).with_context(|| format!("ERR {}.2", day))?;

    Ok(())
}
