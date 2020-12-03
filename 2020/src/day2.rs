use super::util;
use anyhow::{bail, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

struct PasswordLine {
    lower: usize,
    upper: usize,
    letter: char,
    password: String,
    n_letter: usize,
}

impl FromStr for PasswordLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
        }
        let caps = match RE.captures(s) {
            Some(caps) => caps,
            None => bail!("No captures"),
        };
        let lower = get_capture_as_int(&caps, 1)?;
        let upper = get_capture_as_int(&caps, 2)?;
        let letter = get_capture_as_string(&caps, 3)?
            .chars()
            .collect::<Vec<char>>()[0];
        let password = get_capture_as_string(&caps, 4)?;
        let n_letter = match util::count_letters(&password).get(&letter) {
            Some(&n) => n,
            None => 0,
        };

        Ok(PasswordLine {
            lower,
            upper,
            letter,
            password,
            n_letter,
        })
    }
}

impl PasswordLine {
    fn is_valid(&self, part: crate::Part) -> bool {
        match part {
            crate::Part::One => self.n_letter >= self.lower && self.n_letter <= self.upper,
            crate::Part::Two => {
                let has_char_at_lower = match self.password.chars().nth(self.lower - 1) {
                    Some(c) => c == self.letter,
                    None => false,
                };
                let has_char_at_upper = match self.password.chars().nth(self.upper - 1) {
                    Some(c) => c == self.letter,
                    None => false,
                };
                if has_char_at_lower && !has_char_at_upper {
                    true
                } else if !has_char_at_lower && has_char_at_upper {
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl std::fmt::Display for PasswordLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:2}..{:<2} {}'s -- {} ({})",
            self.lower, self.upper, self.letter, self.password, self.n_letter
        )
    }
}

pub fn run(part: crate::Part) -> Result<String> {
    // Part1
    // How many passwords are valid?
    // Format: <m>-<n> <letter>: <password>
    // Must have minimum m <letter>, maximum <n>
    let data = std::fs::read_to_string("input/day2.txt")?;
    let mut valid = 0;

    for line in data.split("\n") {
        if line.trim().len() == 0 {
            continue;
        }
        let pw: PasswordLine = line.parse().unwrap();
        if pw.is_valid(part) {
            valid += 1;
        }
    }
    Ok(format!("Num valid passwords: {}", valid))
}

fn get_capture_as_string(captures: &regex::Captures, idx: usize) -> Result<String> {
    captures
        .get(idx)
        .with_context(|| format!("No lower bound capture"))?
        .as_str()
        .parse()
        .ok()
        .with_context(|| format!("askjdkajs"))
}
fn get_capture_as_int(captures: &regex::Captures, idx: usize) -> Result<usize> {
    Ok(get_capture_as_string(captures, idx)?.parse()?)
}
