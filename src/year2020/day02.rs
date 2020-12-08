use anyhow::Result;

const DAY: usize = 2;

pub fn solve() -> Result<()> {
    let data = std::fs::read_to_string(format!("input/day{}.txt", DAY))?;
    let tidy_data: Vec<_> = data.split('\n').map(|x| x.trim()).collect();
    let passwords: Vec<_> = tidy_data
        .iter()
        .cloned()
        .filter(|x| !x.is_empty())
        .map(|x| parse_password(x))
        .filter_map(|x| x.ok())
        .collect();

    let valid = passwords.iter().filter(|x| valid_rule_part1(x)).count();
    println!("2020 {}-1 -> {}", DAY, valid);

    let valid2 = passwords.iter().filter(|x| valid_rule_part2(x)).count();
    println!("2020 {}-2 -> {}", DAY, valid2);
    Ok(())
}

#[derive(Debug)]
struct PasswordLine<'a> {
    lower: usize,
    upper: usize,
    letter: char,
    password: &'a str,
}

fn parse_password(s: &str) -> Result<PasswordLine> {
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

fn valid_rule_part1(pw: &PasswordLine) -> bool {
    let n_letter = pw.password.chars().filter(|&c| c == pw.letter).count();
    n_letter >= pw.lower && n_letter <= pw.upper
}

macro_rules! bool_xor {
    ($x:expr, $y:expr) => {
        ($x && !$y) || ($y && !$x)
    };
}

fn valid_rule_part2(pw: &PasswordLine) -> bool {
    if pw.lower == 0 || (pw.upper - 1) > pw.password.len() {
        false
    } else {
        let chars: Vec<char> = pw.password.chars().collect();
        let has_char_at_lower = chars
            .get(pw.lower - 1)
            .map(|x| *x == pw.letter)
            .unwrap_or(false);
        let has_char_at_upper = chars
            .get(pw.upper - 1)
            .map(|x| *x == pw.letter)
            .unwrap_or(false);
        bool_xor!(has_char_at_lower, has_char_at_upper)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(
            valid_rule_part1(&parse_password("1-3 a: abcde").unwrap()),
            true
        );
        assert_eq!(
            valid_rule_part1(&parse_password("1-3 b: cdefg").unwrap()),
            false
        );
        assert_eq!(
            valid_rule_part1(&parse_password("2-9 c: ccccccccc").unwrap()),
            true
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            valid_rule_part2(&parse_password("1-3 a: abcde").unwrap()),
            true
        );
        assert_eq!(
            valid_rule_part2(&parse_password("1-3 b: cdefg").unwrap()),
            false
        );
        assert_eq!(
            valid_rule_part2(&parse_password("2-9 c: ccccccccc").unwrap()),
            false
        );
    }
}
