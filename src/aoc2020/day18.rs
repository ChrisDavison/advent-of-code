use anyhow::{anyhow, Result};

pub fn day18() -> Result<()> {
    let data = std::fs::read_to_string("input/18.in")?;

    println!("AoC2020 18.1 -> {}", part1(&data)?);
    // println!("AoC2020 18.2 -> {}", part2(&data)?);
    Ok(())
}

fn part1(data: &str) -> Result<String> {
    let mut sum = 0;
    for line in data.lines() {
        sum += calculate(line)?;
    }
    Ok(format!("{}", sum))
}

fn part2(data: &str) -> Result<String> {
    return Err(anyhow!("Part 2 not implemented"));
    Ok(format!("{}", 0))
}

fn apply(operation: &str, total: i64, num: i64) -> i64 {
    if operation == "+" {
        total + num
    } else {
        total * num
    }
}

fn find_matching_paren(s: &str, start: usize) -> usize {
    let mut count = 1;
    for (idx, c) in s.char_indices().skip(start) {
        if c == ')' {
            count -= 1;
        }
        if c == '(' {
            count += 1;
        }
        if count == 0 {
            return idx;
        }
    }
    return s.len();
}

fn display_eqn_position(eqn: &str, idx_start: usize, idx_end: usize) {
    let mut chars = std::iter::repeat(' ').take(eqn.len()).collect::<Vec<_>>();
    chars[idx_start] = '>';
    chars[idx_end] = '|';
    println!("{}", chars.iter().collect::<String>());
    println!("{}", eqn);
}

// Elf math.
// ...+ and * have equal precedence (e.g just L->R)
// ...evaluate parentheses first
fn calculate(eqn: &str) -> Result<i64> {
    let mut total = 0;
    let mut operation = "+";
    let mut idx = 0;
    while idx < eqn.len() {
        let mut idx2 = eqn[idx..].find(' ').unwrap_or(eqn.len() - 1 - idx) + idx;
        let mut part = "";
        if &eqn[idx..idx + 1] == "(" {
            part = "(";
            idx2 = find_matching_paren(&eqn[idx..], 1) + idx;
        } else {
            part = if idx2 > (eqn.len() - 2) {
                eqn[idx..].trim()
            } else {
                eqn[idx..idx2].trim()
            };
        }

        //display_eqn_position(eqn, idx, idx2);

        if part.chars().all(|c| c.is_digit(10)) {
            total = apply(operation, total, part.parse::<i64>()?);
        } else if part == "(" {
            let sub_eqn_val = apply(operation, total, calculate(&eqn[idx + 1..idx2])?);
            total = sub_eqn_val;
            idx2 = idx2 + 1;
        } else {
            operation = part;
        }

        idx = idx2 + 1;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples_test() {
        let cases = vec![
            ("1 + 2 * 3 + 4 * 5 + 6", 71),
            ("2 * 3 + (4 * 5)", 26),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
            ("((1 + 1) + (1 + 1) + 1) + 1", 6),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
        ];
        for (input, want) in cases {
            assert_eq!(calculate(input).unwrap(), want);
        }
    }

    #[test]
    fn matching_paren_test() {
        assert_eq!(find_matching_paren("()", 1), 1);
        assert_eq!(find_matching_paren("(.)", 1), 2);
        assert_eq!(find_matching_paren("(5 + 5)", 1), 6);
    }
}
