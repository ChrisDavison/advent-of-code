use anyhow::{anyhow, Result};

pub fn day19() -> Result<()> {
    let data = std::fs::read_to_string("input/19.in")?;
    println!("AoC2020 19.1 -> {}", part1(&data)?);
    println!("AoC2020 19.2 -> {}", part1(&data)?);
    Ok(())
}

fn part1(data: &str) -> Result<String> {
    return Err(anyhow!("Part 1 not implemented"));
    Ok(format!("{}", 0))
}

fn part2(data: &str) -> Result<String> {
    return Err(anyhow!("Part 2 not implemented"));
    Ok(format!("{}", 0))
}
