use crate::part::Part;
use anyhow::Result;

pub fn day1(data: &[&str], part: Part) -> Result<()> {
    let lines: Vec<i64> = data
        .iter()
        .map(|x| x.parse())
        .filter_map(|x| x.ok())
        .collect();
    let result = match part {
        Part::One => part1(&lines),
        Part::Two => part2(&lines),
    };
    println!("1.{} - {}", part, result);
    Ok(())
}

pub fn part1(lines: &[i64]) -> i64 {
    let mut result = 0;
    'outer: for (i, el_i) in lines.iter().enumerate() {
        for el_j in &lines[i..] {
            if el_i + el_j == 2020 {
                result = el_i * el_j;
                break 'outer;
            }
        }
    }
    result
}

pub fn part2(lines: &[i64]) -> i64 {
    let mut result = 0;
    'outer: for (i, el_i) in lines.iter().enumerate() {
        for el_j in &lines[i..] {
            for el_k in &lines[i + 1..] {
                if el_i + el_j + el_k == 2020 {
                    result = el_i * el_j * el_k;
                    break 'outer;
                }
            }
        }
    }
    result
}
