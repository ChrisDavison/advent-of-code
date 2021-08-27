use aoc2020::*;

pub fn day18() -> Result<String> {
    let input = include_str!("../input/day18");
    Ok(format!(
        "2020 18.1 -> {}\n2020 18.2 -> {}",
        part1(input)?,
        part2(input)?
    ))
}

fn part1(data: &str) -> Result<String> {
    let mut sum = 0;
    for line in data.lines() {
        sum += calculate(&mut parse_tokens(line))?;
    }
    Ok(format!("{}", sum))
}

fn part2(data: &str) -> Result<String> {
    let mut sum = 0;
    for line in data.lines() {
        sum += calculate2(&mut parse_tokens(line))?;
    }
    Ok(format!("{}", sum))
}

fn find_inner_parens(s: &[Token]) -> (usize, usize) {
    let end = (0..s.len())
        .find(|i| s[*i] == Token::PClose)
        .unwrap_or(s.len());

    let start = (0..end).rfind(|i| s[*i] == Token::POpen).unwrap_or(0);
    (start, end)
}

/// simplify
/// Run calculator on each set of inner parens, returning a new equation string
fn simplify(
    eqn: &mut Vec<Token>,
    calculator: fn(&mut Vec<Token>) -> Result<i64>,
) -> Result<Vec<Token>> {
    while eqn.contains(&Token::POpen) || eqn.contains(&Token::PClose) {
        let (start, end) = find_inner_parens(eqn);
        let sub_eqn = calculator(&mut eqn[start + 1..end].to_vec())?;
        eqn[start] = Token::Num(sub_eqn);
        for i in (start + 1..=end).rev() {
            eqn.remove(i);
        }
    }
    Ok(eqn.clone())
}

/// Elf math
/// ...+ and * have equal precedence (e.g just L->R)
/// ...evaluate parentheses first
fn calculate(eqn: &mut Vec<Token>) -> Result<i64> {
    let mut eqn = simplify(eqn, calculate)?;

    while eqn.contains(&Token::Plus) || eqn.contains(&Token::Mul) {
        let pos_op = eqn
            .iter()
            .position(|x| x == &Token::Plus || x == &Token::Mul)
            .unwrap();
        let before = match eqn[pos_op - 1] {
            Token::Num(n) => n,
            _ => return Err(anyhow!("BEFORE value wasn't a number")),
        };

        let after = match eqn[pos_op + 1] {
            Token::Num(n) => n,
            _ => return Err(anyhow!("AFTER value wasn't a number")),
        };
        let new = if eqn[pos_op] == Token::Plus {
            before + after
        } else {
            before * after
        };
        eqn[pos_op - 1] = Token::Num(new);
        eqn.remove(pos_op + 1);
        eqn.remove(pos_op);
    }
    if eqn.len() == 1 {
        if let Token::Num(val) = eqn[0] {
            Ok(val)
        } else {
            Err(anyhow!("Failed to parse equation"))
        }
    } else {
        Err(anyhow!(
            "Should only be 1 part left, so parsing equation failed. Have '{:?}'",
            eqn
        ))
    }
}

/// Elf math 2.0
/// ...+ his higher precedence than *
/// ...BUT evaluate parentheses first
fn calculate2(eqn: &mut Vec<Token>) -> Result<i64> {
    let mut parts = simplify(eqn, calculate2)?;
    let mut parts = loop_until_symbol_evaluated(&mut parts, Token::Plus, &std::ops::Add::add)?;
    let parts = loop_until_symbol_evaluated(&mut parts, Token::Mul, &std::ops::Mul::mul)?;

    if parts.len() == 1 {
        match parts[0] {
            Token::Num(n) => Ok(n),
            _ => Err(anyhow!("Failed to parse equation")),
        }
    } else {
        Err(anyhow!(
            "Should only be 1 part left, so parsing equation failed. Have '{:?}'",
            parts
        ))
    }
}

fn loop_until_symbol_evaluated<F>(
    parts: &mut Vec<Token>,
    symbol: Token,
    op: &F,
) -> Result<Vec<Token>>
where
    F: FnOnce(i64, i64) -> i64 + Copy,
{
    while parts.contains(&symbol) {
        let pos_op = parts.iter().position(|x| x == &symbol).unwrap();

        let before = match parts[pos_op - 1] {
            Token::Num(n) => n,
            _ => return Err(anyhow!("BEFORE value wasn't a number")),
        };

        let after = match parts[pos_op + 1] {
            Token::Num(n) => n,
            _ => return Err(anyhow!("AFTER value wasn't a number")),
        };

        let new = op(before, after);
        parts[pos_op - 1] = Token::Num(new);
        parts.remove(pos_op + 1);
        parts.remove(pos_op);
    }
    Ok(parts.clone())
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Num(i64),
    Mul,
    Plus,
    POpen,
    PClose,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Token::Num(n) => format!("{}", n),
            Token::Mul => "*".to_string(),
            Token::Plus => "+".to_string(),
            Token::POpen => "(".to_string(),
            Token::PClose => ")".to_string(),
        };
        write!(f, "{}", s)
    }
}

fn parse_tokens(s: &str) -> Vec<Token> {
    s.replace("(", "( ")
        .replace(")", " )")
        .split(' ')
        .map(|w| match w {
            "(" => Token::POpen,
            ")" => Token::PClose,
            "+" => Token::Plus,
            "*" => Token::Mul,
            _ => Token::Num(w.parse::<i64>().unwrap()),
        })
        .collect::<Vec<Token>>()
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
            assert_eq!(calculate(&mut parse_tokens(&input)).unwrap(), want);
        }
    }

    #[test]
    fn token_part1_test() {
        let cases = vec![
            ("1 + 2 * 3 + 4 * 5 + 6", 71),
            ("2 * 3 + (4 * 5)", 26),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
            ("((1 + 1) + (1 + 1) + 1) + 1", 6),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
        ];
        for (input, want) in cases {
            assert_eq!(calculate(&mut parse_tokens(input)).unwrap(), want);
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
            assert_eq!(calculate2(&mut parse_tokens(input)).unwrap(), want);
        }
    }

    #[test]
    fn inner_parens_test() {
        // parens are counted as 'objects'
        // so the found parens are as the resulting vector of enum, not
        // a string index
        // e.g. [1, +, (, 2, +, 3, )] => (2, 6)
        assert_eq!(find_inner_parens(&mut parse_tokens("1 + (2 + 3)")), (2, 6));
        assert_eq!(find_inner_parens(&mut parse_tokens("(2 + 3) + 1")), (0, 4));
    }
}
