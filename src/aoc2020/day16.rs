use anyhow::{anyhow, Result};

pub fn day16() -> Result<()> {
    let data = std::fs::read_to_string("input/16.in")?;
    part1(&data)?;
    part2(&data)?;
    Ok(())
}

fn part1(data: &str) -> Result<()> {
    return Err(anyhow!("Part 1 not implemented"));
    let result = 0;
    println!("AoC2020 16.1 -> {}", result);
    Ok(())
}

fn part2(data: &str) -> Result<()> {
    return Err(anyhow!("Part 2 not implemented"));
    let result = 0;
    println!("AoC2020 16.2 -> {}", result);
    Ok(())
}
