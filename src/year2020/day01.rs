use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashSet;

const DAY: usize = 1;
const TARGET: i32 = 2020;

pub fn day01() -> Result<()> {
    let data = std::fs::read_to_string(format!("input/day{}.in", DAY))?;
    let lines: HashSet<i32> = data
        .as_parallel_string()
        .lines()
        .filter_map(|x| Some(x.trim().parse().ok()?))
        .filter(|&x| x < TARGET)
        .collect();

    let product_of_pair = find_pair_sums_to(TARGET, &lines).map(|(a, b)| a * b);
    if let Some(prod) = product_of_pair {
        println!("2020 {}-1 -> {}", DAY, prod);
    }

    let product_of_triple = find_triple_sums_to(TARGET, &lines).map(|(a, b, c)| a * b * c);
    if let Some(prod) = product_of_triple {
        println!("2020 {}-1 -> {}", DAY, prod);
    }
    Ok(())
}

fn find_pair_sums_to(target: i32, data: &HashSet<i32>) -> Option<(i32, i32)> {
    for num in data {
        if data.contains(&(target - *num)) {
            return Some((*num, target - *num));
        }
    }
    None
}

fn find_triple_sums_to(target: i32, data: &HashSet<i32>) -> Option<(i32, i32, i32)> {
    for num in data {
        if let Some((a, b)) = find_pair_sums_to(target - num, data) {
            return Some((*num, a, b));
        }
    }
    None
}
