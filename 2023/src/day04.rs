#![allow(dead_code, unused_variables, unused_mut, unreachable_code)]
use aoc2023::*;

#[allow(dead_code)]
const SAMPLE: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

pub fn day04() -> Result<String> {
    let input = include_str!("../input/day04");
    // let input = SAMPLE;
    Ok(format!(
        "2023 04.1 -> {}
        2023 04.2 -> {}",
        part1(&input)?,
        part2(&input)?
    ))
}

fn part1(data: &str) -> Result<i32> {
    let mut games: Vec<(usize, HashSet<usize>, HashSet<usize>)> = Vec::new();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Card (\d+): (.*) \| (.*)").unwrap();
    }
    let mut sum = 0;
    for line in data.lines() {
        if let Some(m) = RE.captures(line) {
            games.push((
                m.get(1).unwrap().as_str().parse().unwrap(),
                m.get(2)
                    .unwrap()
                    .as_str()
                    .split(" ")
                    .filter_map(|x| x.trim().parse().ok())
                    .collect(),
                m.get(3)
                    .unwrap()
                    .as_str()
                    .split(" ")
                    .filter_map(|x| x.trim().parse().ok())
                    .collect(),
            ));
            let last = games.last().unwrap();
            let inter = last.1.intersection(&last.2);
            // println!("{:?}", inter);
            let pow = 2_i32.pow((inter.collect::<Vec<_>>().len() - 1) as u32);
            sum += pow;
            // println!("{:?}", pow);
        }
    }
    // println!("{:?}", games);
    println!("{:?}", sum);
    Ok(0)
}

fn part2(data: &str) -> Result<i32> {
    Ok(0)
}
