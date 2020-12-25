#![allow(dead_code, unused_variables, unused_mut, unreachable_code)]
use crate::prelude::*;

pub fn day25() -> Result<()> {
    //let data = SAMPLE;
    let (card, door) = SAMPLE;
    let (card, door) = INPUT;
    println!("AoC2020 25.1 -> {}", part1(card, door)?);
    // println!("AoC2020 25.2 -> {}", part2(&INPUT)?);
    Ok(())
}

fn part1(card: usize, door: usize) -> Result<String> {
    // return Err(anyhow!("Part 1 not implemented"));
    let subj = 7;
    let divi = 20201227;
    let mut start = 1;
    let mut start2 = 1;
    let mut n_iter_card = 0;
    let mut n_iter_door = 0;
    while start != card || start2 != door {
        if start != card {
            start = start * subj % divi;
            n_iter_card += 1;
        }
        if start2 != door {
            start2 = start2 * subj % divi;
            n_iter_door += 1;
        }
    }
    start = 1;
    for i in 0..n_iter_card {
        start = start * door % divi;
    }
    Ok(format!("{}", start))
}

fn part2(data: &str) -> Result<String> {
    return Err(anyhow!("Part 2 not implemented"));
    //Ok(format!("{}", 0))
}

#[allow(dead_code)]
const INPUT: (usize, usize) = (13316116, 13651422);

#[allow(dead_code)]
const SAMPLE: (usize, usize) = (5764801, 17807724);
