#![allow(dead_code, unused_variables, unused_mut, unreachable_code)]
use anyhow::{anyhow, Result};

pub fn day23() -> Result<()> {
    let data = INPUT;
    let n_iter = 100;
    println!("2020 23.1 -> {}", part1(&data, n_iter)?);
    println!("2020 23.2 -> {}", part2(&data)?);
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
struct CupGame {
    current: usize,
    cups: Vec<usize>,
    destination: usize,
    picks: [usize; 3],
}

impl CupGame {
    fn new(data: &[usize]) -> Self {
        Self {
            current: data[0],
            cups: data.to_vec(),
            destination: 0,
            picks: [0; 3],
        }
    }

    fn find_destination_idx(&mut self) -> usize {
        let mut found: usize;
        let mut current = self.current;
        loop {
            // current = (current + self.cups.len() - 1) % self.cups.len();
            // println!("{:?}", self.cups);
            current = if current > *self.cups.iter().min().unwrap() {
                current - 1
            } else {
                *self.cups.iter().max().unwrap()
            };
            if self.cups.contains(&current) {
                found = current;
                break;
            }
        }
        self.destination = found;
        self.cups.iter().position(|d| *d == found).unwrap()
    }

    fn start(&self) -> usize {
        self.cups.iter().position(|c| *c == self.current).unwrap()
    }

    fn position_of(&self, other: usize) -> usize {
        self.cups.iter().position(|c| *c == other).unwrap()
    }

    fn iterate(&mut self) {
        let start_idx = self.start();
        let n = self.cups.len();
        let pick1 = self.cups[(start_idx + 1) % n];
        let pick2 = self.cups[(start_idx + 2) % n];
        let pick3 = self.cups[(start_idx + 3) % n];
        self.picks = [pick1, pick2, pick3];
        for pick in &self.picks {
            self.cups
                .remove(self.cups.iter().position(|d| d == pick).unwrap());
        }
        let dest_idx = self.find_destination_idx();
        self.cups.insert(dest_idx + 1, pick1);
        self.cups.insert(dest_idx + 2, pick2);
        self.cups.insert(dest_idx + 3, pick3);

        self.current = self.cups[(self.start() + 1) % n];
    }

    fn cupstr(&self) -> String {
        self.cups
            .iter()
            .map(|x| {
                if *x == self.current {
                    format!("({})", x)
                } else {
                    format!("{}", x)
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn pickstr(&self) -> String {
        self.picks
            .iter()
            .map(|p| format!("{}", p))
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn from_1(&self) -> String {
        let start = self.cups.iter().position(|c| *c == 1).unwrap();
        (0..self.cups.len() - 1)
            .map(|i| self.cups[(start + i + 1) % self.cups.len()])
            .map(|x| format!("{}", x))
            .collect::<String>()
    }

    fn two_after_1(&self) -> (usize, usize) {
        let start = self.cups.iter().position(|c| *c == 1).unwrap();
        (
            self.cups[(start + 2) % self.cups.len()],
            self.cups[(start + 3) % self.cups.len()],
        )
    }
}

fn part1(data: &[usize], n: usize) -> Result<String> {
    let mut game = CupGame::new(&data);

    for i in 0..n {
        game.iterate();
    }
    Ok(format!("{}", game.from_1()))
}

fn part2(data: &[usize]) -> Result<String> {
    let max = data.iter().max().unwrap();
    let n = data.len();
    let need = 1E6 as usize - n;
    let mut data2: Vec<usize> = (0..1e6 as usize).collect();
    for (i, d) in data.iter().enumerate() {
        data2[i] = *d;
    }
    let mut game = CupGame::new(&data2);
    let n_iter = 10E6 as usize;
    for _ in 0..n_iter {
        game.iterate();
    }

    Ok(format!("{:?}", game.two_after_1()))
}

#[allow(dead_code)]
const INPUT: [usize; 9] = [3, 1, 5, 6, 7, 9, 8, 2, 4];

#[allow(dead_code)]
const SAMPLE: [usize; 9] = [3, 8, 9, 1, 2, 5, 4, 6, 7];
