use crate::part::Part;

use anyhow::{bail, Result};

#[derive(Debug)]
struct PasswordLine<'a> {
    lower: usize,
    upper: usize,
    letter: char,
    password: &'a str,
    n_letter: usize,
}

pub fn day2(data: &[&str], part: Part) -> Result<()> {
    let mut valid = 0;

    let valid_rule_func = match part {
        Part::One => valid_rule_part1,
        Part::Two => valid_rule_part2,
    };

    for line in data {
        if valid_rule_func(parse_password(line)?) {
            valid += 1;
        }
    }
    println!("2.{} - {}", part, valid);
    Ok(())
}

fn recursive_chunk<'a>(s: &'a str, next_stop: char) -> (&'a str, &'a str) {
    if let Some(idx) = s.find(next_stop) {
        (s[0..idx].trim(), s[idx + 1..].trim())
    } else {
        (s, "")
    }
}

fn parse_password<'a>(s: &str) -> Result<PasswordLine> {
    if s.trim().len() == 0 {
        bail!("Empty string")
    }
    let (lower, s) = recursive_chunk(s, '-');
    let (upper, s) = recursive_chunk(s, ' ');
    let (letter, s) = recursive_chunk(s, ':');
    let ch = letter.parse()?;
    let n_letter = s.chars().filter(|&c| c == ch).collect::<Vec<char>>().len();

    Ok(PasswordLine {
        lower: lower.parse()?,
        upper: upper.parse()?,
        letter: ch,
        password: s,
        n_letter: n_letter,
    })
}

fn valid_rule_part1(pw: PasswordLine) -> bool {
    pw.n_letter >= pw.lower && pw.n_letter <= pw.upper
}

fn valid_rule_part2(pw: PasswordLine) -> bool {
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
