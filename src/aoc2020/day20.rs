use anyhow::{anyhow, Result};

pub fn day20() -> Result<()> {
    let data = std::fs::read_to_string("input/20.in")?;
    println!("AoC2020 20.1 -> {}", part1(&data)?);
    println!("AoC2020 20.2 -> {}", part2(&data)?);
    Ok(())
}

fn part1(data: &str) -> Result<String> {
    Err(anyhow!("Part 1 not implemented"))
    //Ok(format!("{}", 0))
}

fn part2(data: &str) -> Result<String> {
    Err(anyhow!("Part 2 not implemented"))
    //Ok(format!("{}", 0))
}
