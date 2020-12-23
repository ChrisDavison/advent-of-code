use crate::prelude::*;

const N_ITER_P1: usize = 100;
const N_ITER_P2: usize = 10_000_000;

pub fn day23() -> Result<()> {
    let data = INPUT;
    println!("2020 23.1 -> {}", part1(&data, N_ITER_P1)?);
    println!("2020 23.2 -> {}", part2(&data, N_ITER_P2)?);
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
struct CupGame {
    current: usize,
    pointed_at_by: Vec<usize>,
    min: usize,
    max: usize,
}

impl CupGame {
    fn new(data: &[usize]) -> Self {
        let mut rotated = data.to_vec();
        rotated.rotate_left(1);
        let mut ring = vec![0; data.len() + 1];
        for (idx, after) in data.iter().zip(rotated.iter()) {
            ring[*idx] = *after;
        }
        Self {
            current: data[0],
            pointed_at_by: ring,
            min: *data.iter().min().unwrap(),
            max: *data.iter().max().unwrap(),
        }
    }

    fn find_destination(&mut self) -> usize {
        let dec_wrap = |current| {
            if current > self.min {
                current - 1
            } else {
                self.max
            }
        };

        let n = self.pointed_at_by[self.current];
        let nn = self.pointed_at_by[n];
        let nnn = self.pointed_at_by[nn];
        let mut current = dec_wrap(self.current);
        while [n, nn, nnn].contains(&current) {
            current = dec_wrap(current);
        }
        current
    }

    #[allow(dead_code)]
    fn display_ring(&self) -> String {
        let mut c = self.current;
        c = self.pointed_at_by[c];
        let mut picks = vec![];
        for _ in 0..3 {
            picks.push(c);
            c = self.pointed_at_by[c];
        }
        let mut rest = vec![];
        loop {
            if c == self.current {
                break;
            }
            rest.push(c);
            c = self.pointed_at_by[c];
        }
        let s = picks
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .join(" ");
        let s2 = rest
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .join(" ");

        format!("{} [{}] [{}]", self.current, s, s2)
    }

    fn iterate(&mut self) {
        let destination = self.find_destination();
        let n = self.pointed_at_by[self.current];
        let nn = self.pointed_at_by[n];
        let nnn = self.pointed_at_by[nn];
        let after_destination = self.pointed_at_by[destination];

        self.pointed_at_by[self.current] = self.pointed_at_by[nnn];
        self.pointed_at_by[destination] = n;
        self.pointed_at_by[nnn] = after_destination;
        self.current = self.pointed_at_by[self.current];
    }

    fn score_p1(&self) -> String {
        let mut c = 1;
        let mut out = vec![];
        loop {
            c = self.pointed_at_by[c];
            if c == 1 {
                break;
            }
            out.push(format!("{}", c));
        }
        out.join("")
    }

    fn score_p2(&self) -> usize {
        let after_1 = self.pointed_at_by[1];
        let after_after_1 = self.pointed_at_by[after_1];
        after_1 * after_after_1
    }
}

fn part1(data: &[usize], n: usize) -> Result<String> {
    let mut game = CupGame::new(&data);
    for _ in 0..n {
        game.iterate();
    }
    Ok(format!("{}", game.score_p1()))
}

fn part2(data: &[usize], n: usize) -> Result<String> {
    let mut data = data.to_vec();
    data.extend(data.len() + 1..=1E6 as usize);
    let mut game = CupGame::new(&data);
    for _ in 0..n {
        game.iterate();
    }

    Ok(format!("{:?}", game.score_p2()))
}

#[allow(dead_code)]
const INPUT: [usize; 9] = [3, 1, 5, 6, 7, 9, 8, 2, 4];

#[allow(dead_code)]
const SAMPLE: [usize; 9] = [3, 8, 9, 1, 2, 5, 4, 6, 7];

#[allow(dead_code)]
const SAMPLE2: [usize; 5] = [5, 4, 3, 2, 1];
