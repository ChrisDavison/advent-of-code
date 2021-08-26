use aoc2020::*;

pub fn day25() -> Result<()> {
    // let (card, door) = SAMPLE;
    let input = include_str!("../input/day25");
    let input = parse_each(input.lines());
    let (card, door) = (input[0], input[1]);
    println!("2020 25 -> {}", run(card, door)?);
    Ok(())
}

fn run(card: usize, door: usize) -> Result<String> {
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
    Ok(format!("{}", start))
}

#[allow(dead_code)]
const SAMPLE: (usize, usize) = (5764801, 17807724);
