use crate::part::Part;

use anyhow::Result;

#[derive(Debug)]
struct PasswordLine<'a> {
    lower: usize,
    upper: usize,
    letter: char,
    password: &'a str,
}

pub fn day2(data: &[&str], part: Part) -> Result<()> {
    let mut valid = 0;

    let valid_rule_func = match part {
        Part::One => valid_rule_part1,
        Part::Two => valid_rule_part2,
    };

    for line in data.iter().filter(|x| x.len() != 0) {
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
    let split_chars = " :-";
    let parts: Vec<&str> = s
        .split(|x| split_chars.contains(x))
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect();

    Ok(PasswordLine {
        lower: parts[0].parse()?,
        upper: parts[1].parse()?,
        letter: parts[2].parse()?,
        password: parts[3],
    })
}

fn valid_rule_part1(pw: PasswordLine) -> bool {
    let n_letter = pw
        .password
        .chars()
        .filter(|&c| c == pw.letter)
        .collect::<Vec<char>>()
        .len();
    n_letter >= pw.lower && n_letter <= pw.upper
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

#[allow(dead_code, unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_part1_examples() {
        assert_eq!(
            valid_rule_part1(parse_password("1-3 a: abcde").unwrap()),
            true
        );
        assert_eq!(
            valid_rule_part1(parse_password("1-3 b: cdefg").unwrap()),
            false
        );
        assert_eq!(
            valid_rule_part1(parse_password("2-9 c: ccccccccc").unwrap()),
            true
        );
    }

    #[test]
    fn test_day2_part2_examples() {
        assert_eq!(
            valid_rule_part2(parse_password("1-3 a: abcde").unwrap()),
            true
        );
        assert_eq!(
            valid_rule_part2(parse_password("1-3 b: cdefg").unwrap()),
            false
        );
        assert_eq!(
            valid_rule_part2(parse_password("2-9 c: ccccccccc").unwrap()),
            false
        );
    }
}
