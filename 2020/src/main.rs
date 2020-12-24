#![feature(core_intrinsics)]
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

mod bool_xor;
mod dict;
mod prelude;
mod strides;
mod time_macro;
extern crate lazy_static;

const SOLUTIONS: &[fn() -> anyhow::Result<()>] = &[
    day01::day01,
    day02::day02,
    day03::day03,
    day04::day04,
    day05::day05,
    day06::day06,
    day07::day07,
    day08::day08,
    day09::day09,
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
];

fn main() {
    let day_wanted = std::env::args()
        .nth(1)
        .map(|x| x.parse::<usize>().ok())
        .flatten();
    for (day, function) in SOLUTIONS.iter().enumerate() {
        if let Some(d) = day_wanted {
            if day != d - 1 {
                continue;
            }
        }
        time_solution!(day, function());
    }
}
