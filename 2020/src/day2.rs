use anyhow::{bail, Result};
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
        if s.trim().len() == 0 {
            bail!("Empty string")
        }
        let idx_dash = s.find("-").unwrap();
        let idx_space_after_dash = s.find(" ").unwrap();
        let idx_colon = s.find(":").unwrap();

        let lower: usize = s[0..idx_dash].parse()?;
        let upper: usize = s[idx_dash + 1..idx_space_after_dash].parse()?;
        let letter: char = s[idx_space_after_dash + 1..idx_space_after_dash + 2].parse()?;
        let password: String = s[idx_colon + 2..].parse().unwrap();
        let n_letter = password
            .chars()
            .filter(|&c| c == letter)
            .collect::<Vec<char>>()
            .len();

        Ok(PasswordLine {
            lower,
            upper,
            letter,
            password,
            n_letter,
        })
    }
}

pub fn run(data: &String, part: crate::Part) -> Result<String> {
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
    Ok(format!("{}", valid))
}

fn is_valid(pw: PasswordLine, part: crate::Part) -> bool {
    match part {
        crate::Part::One => pw.n_letter >= pw.lower && pw.n_letter <= pw.upper,
        crate::Part::Two => {
            let chars: Vec<char> = pw.password.chars().collect();
            let has_char_at_lower = chars
                .iter()
                .nth(pw.lower - 1)
                .and_then(|&c| Some(c == pw.letter))
                .unwrap_or(false);
            let has_char_at_upper = chars
                .iter()
                .nth(pw.upper - 1)
                .and_then(|&c| Some(c == pw.letter))
                .unwrap_or(false);
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
