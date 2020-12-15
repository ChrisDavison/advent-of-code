use anyhow::Result;

pub fn dayXX() -> Result<()> {
    let data = std::fs::read_to_string("input/XX.in")?;
    part_1(&data)?;
    part_2(&data)?;
    Ok(())
}

fn part_1(data: &str) -> Result<()> {
    unimplemented!();
    let result = 0;
    println!("AoC2020 XX.1 -> {}", result);
    Ok(())
}

fn part_2(data: &str) -> Result<()> {
    unimplemented!();
    let result = 0;
    println!("AoC2020 XX.2 -> {}", result);
    Ok(())
}
