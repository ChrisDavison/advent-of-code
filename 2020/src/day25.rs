use crate::prelude::*;

pub fn day25() -> Result<()> {
    // let (card, door) = SAMPLE;
    let (card, door) = INPUT;
    let subj = 7;
    let divi = 20201227;
    let mut start = 1;
    let subj2: usize;
    let mut iters2 = 0;
    loop {
        if start == card {
            subj2 = door;
            break;
        }
        if start == door {
            subj2 = card;
            break;
        }
        start = start * subj % divi;
        iters2 += 1;
    }
    start = 1;
    for _ in 0..iters2 {
        start = start * subj2 % divi;
    }

    println!("AoC2020 25 -> {}", start);
    Ok(())
}

#[allow(dead_code)]
const INPUT: (usize, usize) = (13316116, 13651422);

#[allow(dead_code)]
const SAMPLE: (usize, usize) = (5764801, 17807724);
