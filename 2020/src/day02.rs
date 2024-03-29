use aoc2020::*;

pub fn day02() -> Result<String> {
    let data = include_str!("../input/day02");
    let passwords: Vec<_> = data
        .lines()
        .filter_map(|x| parse_password(x.trim()).ok())
        .collect();

    let valid = part1(&passwords);
    let valid2 = part2(&passwords);

    let output = format!("2020 02.1 -> {}\n2020 02.2 -> {}", valid, valid2);
    Ok(output)
}

fn part1(passwords: &[PasswordLine]) -> usize {
    passwords.iter().filter(|x| valid_rule_part1(x)).count()
}

fn part2(passwords: &[PasswordLine]) -> usize {
    passwords.iter().filter(|x| valid_rule_part2(x)).count()
}

#[derive(Debug)]
struct PasswordLine<'a> {
    lower: usize,
    upper: usize,
    letter: char,
    password: &'a str,
}

fn parse_password(s: &str) -> Result<PasswordLine> {
    let mut parts = s
        .split(|x| " :-".contains(x))
        .map(|x| x.trim())
        .filter(|x| !x.is_empty());

    Ok(PasswordLine {
        lower: parts
            .next()
            .ok_or_else(|| anyhow!("No lower"))
            .and_then(|x| x.parse().map_err(|_| anyhow!("Bad lower")))?,
        upper: parts
            .next()
            .ok_or_else(|| anyhow!("No upper"))
            .and_then(|x| x.parse().map_err(|_| anyhow!("Bad upper")))?,
        letter: parts
            .next()
            .ok_or_else(|| anyhow!("No letter"))
            .and_then(|x| x.parse().map_err(|_| anyhow!("Bad letter")))?,
        password: parts.next().ok_or_else(|| anyhow!("No password"))?,
    })
}

fn valid_rule_part1(pw: &PasswordLine) -> bool {
    let n_letter = pw.password.chars().filter(|&c| c == pw.letter).count();
    n_letter >= pw.lower && n_letter <= pw.upper
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
#[allow(unused_imports)]
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
