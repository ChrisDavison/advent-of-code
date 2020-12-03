use super::util;
use anyhow::{bail, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
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
        let lower: usize = get_capture(&caps, 1)?.parse()?;
        let upper: usize = get_capture(&caps, 2)?.parse()?;
        let letter = get_capture(&caps, 3)?.chars().collect::<Vec<char>>()[0];
        let password = get_capture(&caps, 4)?;
        let n_letter = util::count_letters(&password)
            .get(&letter)
            .unwrap_or_else(|| &0)
            .to_owned();

        Ok(PasswordLine {
            lower,
            upper,
            letter,
            password,
            n_letter,
        })
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
        if is_valid(pw, part) {
            valid += 1;
        }
    }
    Ok(format!("Num valid passwords: {}", valid))
}

fn get_capture(captures: &regex::Captures, idx: usize) -> Result<String> {
    captures
        .get(idx)
        .with_context(|| format!("No lower bound capture"))?
        .as_str()
        .parse()
        .ok()
        .with_context(|| format!("askjdkajs"))
}

fn is_valid(pw: PasswordLine, part: crate::Part) -> bool {
    match part {
        crate::Part::One => pw.n_letter >= pw.lower && pw.n_letter <= pw.upper,
        crate::Part::Two => {
            let has_char_at_lower = match pw.password.chars().nth(pw.lower - 1) {
                Some(c) => c == pw.letter,
                None => false,
            };
            let has_char_at_upper = match pw.password.chars().nth(pw.upper - 1) {
                Some(c) => c == pw.letter,
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
