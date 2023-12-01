use std::fmt::format;

use aoc2023::*;

pub fn day01() -> Result<String> {
    let data = include_str!("../input/day01");
    // let data = TEST_INPUT;

    let output = format!("2023 01.1 -> {}\n2023 01.2 -> {}", part1(data)?, part2(data)?);
    Ok(output)
}

fn part1(data: &str) -> Result<String> {
    let mut sum = 0;
    for line in data.lines() {
        let nums: Vec<u32> = line.chars().filter(|&x| (x >= '0' && x <= '9')).map(|x| x.to_digit(10).unwrap()).collect();
        let first = nums[0];
        let last = nums[nums.len()-1];
        let num = 10*first + last;
        sum += num;
    }
    Ok(format!("{}", sum))
}

fn part2(data: &str) -> Result<String> {
    // todo!("part1")
    Ok("unimplemented".to_string())
}

#[allow(dead_code)]
const TEST_INPUT: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
