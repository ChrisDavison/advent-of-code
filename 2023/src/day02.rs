use aoc2023::*;

pub fn day02() -> Result<String> {
    // let data = include_str!("../input/day02");
    let data = TEST_INPUT;

    let output = format!(
        "2023 02.1 -> {}\n2023 02.2 -> {}",
        part1(data)?,
        part2(data)?
    );
    Ok(output)
}

fn part1(data: &str) -> Result<u32> {
    todo!("part1")
}

fn part2(data: &str) -> Result<u32> {
    todo!("part1")
}

#[allow(dead_code)]
const TEST_INPUT: &str = r"";

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
