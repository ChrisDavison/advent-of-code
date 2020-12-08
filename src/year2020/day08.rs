use anyhow::Result;
use std::collections::HashMap;

const DAY: usize = 8;

pub fn solve() -> Result<()> {
    let data = std::fs::read_to_string(format!("input/day{}.txt", DAY))?;
    let tidy_data: Vec<&str> = data
        .split('\n')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect();

    let (total, total2) = (-1, -1);

    println!("AoC2020 {}.1 - {}", DAY, total);
    println!("AoC2020 {}.2 - {}", DAY, total2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
