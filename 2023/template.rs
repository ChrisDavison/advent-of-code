use aoc2023::*;

#[allow(dead_code)]
const SAMPLE: &str = r"";

const DATA: &str = include_str!("../input/dayDAYNUM2");

const USE_SAMPLE: bool = true;

fn main() {
    let data = if USE_SAMPLE { SAMPLE } else { DATA };
    timed! {DAYNUM1, part1, data};
    timed! {DAYNUM1, part2, data};
}

fn part1(data: &str) -> Result<usize> {
    Err(anyhow!("Not implemented"))
}

fn part2(data: &str) -> Result<usize> {
    Err(anyhow!("Not implemented"))
}
