use anyhow::Result;

const DAY: usize = 0;

fn main() -> Result<()> {
    let data = std::fs::read_to_string(format!("input/day{}.txt", DAY))?;
    let tidy_data: Vec<&str> = data.lines().map(|x| x.trim()).collect();

    let result1 = 0;
    let result2 = 0;
    println!("AoC2020 {}.1 - {}", DAY, result1);
    println!("AoC2020 {}.2 - {}", DAY, result2);
    Ok(())
}

#[cfg(test)]
mod tests_dayNN {
    use super::*;

    #[test]
    fn template_day_was_changed() {
        assert_ne!(DAY, 0);
    }
    #[test]
    fn examples_part1() {
        let tests = vec![()];
        for (input, expected) in tests {
            assert_eq!(...(input), expected);
        }
    }

    #[test]
    fn examples_part2() {
        let tests = vec![()];
        for (input, expected) in tests {
            assert_eq!(...(input), expected);
        }
    }
}
