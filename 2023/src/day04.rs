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
    // let SAMPLE = SAMPLE;
    Ok(format!(
        "2023 04.1 -> {}\n2023 04.2 -> {}",
        part1(input)?,
        part2(input)?
    ))
}

struct Game {
    id: usize,
    winning: Vec<usize>,
    have: Vec<usize>,
}

fn numbers(s: &str) -> Vec<usize> {
    s.split(" ").filter_map(|x| x.trim().parse().ok()).collect()
}

impl Game {
    fn new(line: &str) -> Option<Game> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Card\s+(\d+): (.*) \| (.*)").unwrap();
        }
        RE.captures(line).map(|m| Game {
            id: m.get(1).unwrap().as_str().parse().unwrap(),
            winning: numbers(m.get(2).unwrap().as_str()),
            have: numbers(m.get(3).unwrap().as_str()),
        })
    }
}

fn parse(data: &str) -> Vec<Game> {
    data.lines().filter_map(Game::new).collect()
}

fn part1(data: &str) -> Result<i32> {
    let mut sum = 0;
    for game in parse(data) {
        let haveset = game.have.iter().collect::<HashSet<_>>();
        let winset = game.winning.iter().collect::<HashSet<_>>();
        let inter = haveset.intersection(&winset).collect::<Vec<_>>();
        if inter.is_empty() {
            continue;
        }
        let pow = 2_i32.pow(inter.len() as u32 - 1);
        sum += pow;
    }
    Ok(sum)
}

fn part2(data: &str) -> Result<usize> {
    let mut sum = 0;
    let parsed = parse(data);
    let mut copies_of_each: Vec<usize> = Vec::with_capacity(parsed.len() + 1);
    copies_of_each.resize(parsed.len() + 1, 1);
    copies_of_each[0] = 0;

    for game in &parsed {
        let haveset = game.have.iter().collect::<HashSet<_>>();
        let winset = game.winning.iter().collect::<HashSet<_>>();
        let inter = haveset.intersection(&winset).collect::<Vec<_>>();
        if inter.is_empty() {
            continue;
        }
        let games_to_add = (1..=inter.len()).map(|x| game.id + x).collect::<Vec<_>>();
        for g in &games_to_add {
            copies_of_each[*g] += copies_of_each[game.id];
        }
    }
    Ok(copies_of_each.iter().sum())
}
