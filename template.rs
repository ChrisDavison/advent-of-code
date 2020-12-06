use anyhow::Result;

const DAY: usize = _;

fn main() -> Result<()> {
    let data = std::fs::read_to_string(format!("input/day{}.txt", DAY))?;
    let tidy_data: Vec<&str> = data.split("\n").map(|x| x.trim()).collect();

    let result1 = _;
    let result2 = _;
    println!("AoC2020 {}.1 - {}", DAY, result1);
    println!("AoC2020 {}.2 - {}", DAY, result2);
    Ok(())
}

#[cfg(test)]
mod tests_dayNN {
    #[test]
    fn examples_part1() {
        let tests = vec![
            (),
        ];
        for (input, expected) in tests {
            assert_eq!(...(input), expected);
        }
    }

    #[test]
    fn examples_part2() {
        let tests = vec![
            (),
        ];
        for (input, expected) in tests {
            assert_eq!(...(input), expected);
        }
    }
}
