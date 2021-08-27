use aoc2020::*;

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

fn main() {
    let start = std::time::Instant::now();

    time_solution! {1, day01};
    time_solution! {2, day02};
    time_solution! {3, day03};
    time_solution! {4, day04};
    time_solution! {5, day05};
    time_solution! {6, day06};
    time_solution! {7, day07};
    time_solution! {8, day08};
    time_solution! {9, day09};
    time_solution! {10, day10};
    time_solution! {11, day11};
    time_solution! {12, day12};
    time_solution! {13, day13};
    time_solution! {14, day14};
    time_solution! {15, day15};
    time_solution! {16, day16};
    time_solution! {17, day17};
    time_solution! {18, day18};
    time_solution! {19, day19};
    time_solution! {20, day20};
    time_solution! {21, day21};
    time_solution! {22, day22};
    time_solution! {23, day23};
    time_solution! {24, day24};
    time_solution! {25, day25};

    println!(
        "TOTAL TIME: {:.2}ms",
        start.elapsed().as_nanos() / 1_000_000
    );
}
