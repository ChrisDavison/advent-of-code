use aoc2023::*;

lazy_static! {
    static ref RE: Regex = Regex::new("(one|two|three|four|five|six|seven|eight|nine|[0-9])")
        .expect("Failed to construct regex");
}

pub fn day01() -> Result<(String, String)> {
    let data = include_str!("../input/day01");

    Ok((part1(data)?.to_string(), part2(data)?.to_string()))
}

type DigitFinder = fn(&str) -> Vec<u32>;

fn digits_in_line(line: &str) -> Vec<u32> {
    line.chars().filter_map(|x| x.to_digit(10)).collect()
}

fn digits_and_worddigits_in_line(line: &str) -> Vec<u32> {
    let mut nums = vec![];
    let mut ranges_seen: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..line.len() {
        if let Some(m) = RE.find(&line[i..]) {
            let r = (m.start() + i, m.end() + i);
            let unseen = ranges_seen.insert(r);
            if unseen {
                nums.push(word_to_num(m.as_str()));
            }
        }
    }
    nums
}

fn part1(data: &str) -> Result<u32> {
    Ok(sum_of_combined_first_last_digit(data, digits_in_line))
}

fn part2(data: &str) -> Result<u32> {
    Ok(sum_of_combined_first_last_digit(
        data,
        digits_and_worddigits_in_line,
    ))
}

fn sum_of_combined_first_last_digit(data: &str, finder: DigitFinder) -> u32 {
    data.lines()
        .map(finder)
        .filter(|nums| !nums.is_empty())
        .map(|nums| nums[0] * 10 + nums[nums.len() - 1])
        .sum()
}

fn word_to_num(s: &str) -> u32 {
    match s {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        "zero" | "0" => 0,
        _ => 0,
    }
}

#[allow(dead_code)]
const TEST_INPUT: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

#[allow(dead_code)]
const TEST_INPUT_2: &str = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_digits_in_line() {
        assert_eq!(digits_in_line("abc123"), [1, 2, 3]);
        assert_eq!(digits_in_line("a1bc13"), [1, 1, 3]);
        assert_eq!(digits_in_line("abc"), []);
        assert_eq!(digits_in_line("ab1c"), [1]);
        assert_eq!(digits_and_worddigits_in_line("ab1ctwo"), [1, 2]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            sum_of_combined_first_last_digit(TEST_INPUT, digits_in_line),
            142
        );
        assert_eq!(
            sum_of_combined_first_last_digit(TEST_INPUT_2, digits_in_line),
            209
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            sum_of_combined_first_last_digit(TEST_INPUT, digits_and_worddigits_in_line),
            142
        );
        assert_eq!(
            sum_of_combined_first_last_digit(TEST_INPUT_2, digits_and_worddigits_in_line),
            281
        );
    }
}
