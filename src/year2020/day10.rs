use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashMap;

const DAY: usize = 10;

pub fn day10(data: &str) -> Result<()> {
    let mut data: Vec<usize> = data
        .as_parallel_string()
        .lines()
        .filter_map(|x| Some(x.parse().ok()?))
        .collect();
    data.sort_unstable();
    part_1(&data);
    part_2(&data);
    Ok(())
}

fn part_1(data: &[usize]) -> usize {
    // start with (1,1) in acc, as the wall-socket is a 1, and our device is a 3
    let (ones, threes) =
        data.windows(2)
            .fold((1, 1), |(ones, threes), val| match val[1] - val[0] {
                1 => (ones + 1, threes),
                3 => (ones, threes + 1),
                _ => unreachable!(),
            });
    println!("AoC2020 {}.1 -> {}", DAY, ones * threes);
    ones * threes
}

fn part_2(data: &[usize]) -> usize {
    let result = data.windows(2).collect::<Vec<_>>();
    let split = result.split(|n| n[1] - n[0] == 3).collect::<Vec<_>>();
    for s in &split {
        println!("{} {:?}", s.len(), s);
    }
    let product = 2 * split
        .iter()
        .map(|n| match n.len() {
            4 => 7,
            3 => 4,
            2 => 2,
            _ => 1,
        })
        .product::<usize>();
    println!("AoC2020 {}.2 -> {}", DAY, product);
    product
}
