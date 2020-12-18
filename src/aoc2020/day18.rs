use anyhow::{anyhow, Result};

pub fn day18() -> Result<()> {
    let data = std::fs::read_to_string("input/18.in")?;

    println!("AoC2020 18.1 -> {}", part1(&data)?);
    println!("AoC2020 18.2 -> {}", part2(&data)?);
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
    let mut sum = 0;
    for line in data.lines() {
        sum += calculate2(line)?;
    }
    Ok(format!("{}", sum))
}

fn find_inner_parens(s: &str) -> (usize, usize) {
    let end = s.find(')').unwrap_or(s.len());
    let start = s[..end].rfind('(').unwrap_or(0);
    (start, end)
}

/// simplify
/// Run calculator on each set of inner parens, returning a new equation string
fn simplify(eqn: impl ToString, calculator: fn(String) -> Result<i64>) -> Result<String> {
    // Simplify all inner parens
    let mut eqn = eqn.to_string();
    while eqn.contains("(") || eqn.contains(")") {
        let (start, end) = find_inner_parens(&eqn);
        let sub_eqn = format!("{}", calculator(eqn[start + 1..end].to_string())?);
        eqn = format!("{}{}{}", &eqn[..start], sub_eqn, &eqn[end + 1..]).to_string();
    }
    Ok(eqn.to_string())
}

/// Elf math
/// ...+ and * have equal precedence (e.g just L->R)
/// ...evaluate parentheses first
fn calculate(eqn: impl ToString) -> Result<i64> {
    let eqn = simplify(eqn.to_string(), calculate)?;

    let mut parts = eqn.split(" ").map(|x| x.to_string()).collect::<Vec<_>>();

    while parts.contains(&String::from("+")) || parts.contains(&String::from("*")) {
        let pos_op = parts.iter().position(|x| x == &"+" || x == &"*").unwrap();
        let new = if parts[pos_op] == "+" {
            parts[pos_op - 1].parse::<i64>()? + parts[pos_op + 1].parse::<i64>()?
        } else {
            parts[pos_op - 1].parse::<i64>()? * parts[pos_op + 1].parse::<i64>()?
        };
        parts[pos_op - 1] = format!("{}", new);
        parts.remove(pos_op + 1);
        parts.remove(pos_op);
    }
    if parts.len() == 1 {
        parts[0]
            .parse::<i64>()
            .map_err(|_| anyhow!("Failed to parse equation"))
    } else {
        Err(anyhow!(
            "Should only be 1 part left, so parsing equation failed. Have '{:?}'",
            parts
        ))
    }
}

/// Elf math 2.0
/// ...+ his higher precedence than *
/// ...BUT evaluate parentheses first
fn calculate2(eqn: impl ToString) -> Result<i64> {
    let eqn = simplify(eqn.to_string(), calculate2)?;

    let mut parts: Vec<String> = eqn.split(' ').map(|x| x.to_string()).collect();
    loop_until_symbol_evaluated(&mut parts, String::from("+"), &std::ops::Add::add)?;
    loop_until_symbol_evaluated(&mut parts, String::from("*"), &std::ops::Mul::mul)?;

    if parts.len() == 1 {
        parts[0]
            .parse::<i64>()
            .map_err(|_| anyhow!("Failed to parse equation"))
    } else {
        Err(anyhow!(
            "Should only be 1 part left, so parsing equation failed. Have '{:?}'",
            parts
        ))
    }
}

fn loop_until_symbol_evaluated<F>(parts: &mut Vec<String>, symbol: String, op: &F) -> Result<()>
where
    F: FnOnce(i64, i64) -> i64 + Copy,
{
    while parts.contains(&symbol) {
        let pos_op = parts.iter().position(|x| x == &symbol).unwrap();
        let new = op(
            parts[pos_op - 1].parse::<i64>()?,
            parts[pos_op + 1].parse::<i64>()?,
        );
        parts[pos_op - 1] = format!("{}", new);
        parts.remove(pos_op + 1);
        parts.remove(pos_op);
    }
    Ok(())
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
    fn part2_examples_test() {
        let cases = vec![
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 46),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
        ];
        for (input, want) in cases {
            assert_eq!(calculate2(input).unwrap(), want);
        }
    }

    #[test]
    fn inner_parens_test() {
        assert_eq!(find_inner_parens("1 + (2 + 3)"), (4, 10));
        assert_eq!(find_inner_parens("(2 + 3) + 1"), (0, 6));
    }
}
