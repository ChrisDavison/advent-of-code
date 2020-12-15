use anyhow::Result;
use std::collections::HashSet;

const TARGET: i32 = 2020;

pub fn day01() -> Result<()> {
    let data = std::fs::read_to_string("input/01.in")?;
    let lines: HashSet<i32> = data
        .lines()
        .filter_map(|x| Some(x.trim().parse().ok()?))
        .filter(|&x| x < TARGET)
        .collect();

    let product_of_pair = find_pair_sums_to(TARGET, &lines).map(|(a, b)| a * b);
    if let Some(prod) = product_of_pair {
        println!("2020 1-1 -> {}", prod);
    }

    let product_of_triple = find_triple_sums_to(TARGET, &lines).map(|(a, b, c)| a * b * c);
    if let Some(prod) = product_of_triple {
        println!("2020 1-1 -> {}", prod);
    }
    Ok(())
}

fn find_pair_sums_to(target: i32, data: &HashSet<i32>) -> Option<(i32, i32)> {
    for num in data {
        let need = target - *num;
        if data.contains(&need) {
            return Some((*num, need));
        }
    }
    None
}

fn find_triple_sums_to(target: i32, data: &HashSet<i32>) -> Option<(i32, i32, i32)> {
    for num in data {
        let need = target - num;
        if let Some((a, b)) = find_pair_sums_to(need, data) {
            return Some((*num, a, b));
        }
    }
    None
}
