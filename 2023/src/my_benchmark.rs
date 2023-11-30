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

use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day01", |b| b.iter(|| day01::day01()));
    c.bench_function("day02", |b| b.iter(|| day02::day02()));
    c.bench_function("day03", |b| b.iter(|| day03::day03()));
    c.bench_function("day04", |b| b.iter(|| day04::day04()));
    c.bench_function("day05", |b| b.iter(|| day05::day05()));
    c.bench_function("day06", |b| b.iter(|| day06::day06()));
    c.bench_function("day07", |b| b.iter(|| day07::day07()));
    c.bench_function("day08", |b| b.iter(|| day08::day08()));
    c.bench_function("day09", |b| b.iter(|| day09::day09()));
    c.bench_function("day10", |b| b.iter(|| day10::day10()));
    c.bench_function("day11", |b| b.iter(|| day11::day11()));
    c.bench_function("day12", |b| b.iter(|| day12::day12()));
    c.bench_function("day13", |b| b.iter(|| day13::day13()));
    c.bench_function("day14", |b| b.iter(|| day14::day14()));
    c.bench_function("day15", |b| b.iter(|| day15::day15()));
    c.bench_function("day16", |b| b.iter(|| day16::day16()));
    c.bench_function("day17", |b| b.iter(|| day17::day17()));
    c.bench_function("day18", |b| b.iter(|| day18::day18()));
    c.bench_function("day19", |b| b.iter(|| day19::day19()));
    c.bench_function("day20", |b| b.iter(|| day20::day20()));
    c.bench_function("day21", |b| b.iter(|| day21::day21()));
    c.bench_function("day22", |b| b.iter(|| day22::day22()));
    c.bench_function("day23", |b| b.iter(|| day23::day23()));
    c.bench_function("day24", |b| b.iter(|| day24::day24()));
    c.bench_function("day25", |b| b.iter(|| day25::day25()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
