use aoc2023::*;

pub fn day01() -> Result<String> {
    let data = include_str!("../input/day01");
    let testdata = TEST_INPUT_2;

    let output = format!(
        "2023 01.1 -> {}\n2023 01.2 -> {}",
        part1(data)?,
        part2(data)?
    );
    Ok(output)
}

fn part1(data: &str) -> Result<String> {
    let mut sum = 0;
    for line in data.lines() {
        let nums: Vec<u32> = line
            .chars()
            .filter(|&x| (x >= '0' && x <= '9'))
            .map(|x| x.to_digit(10).unwrap())
            .collect();
        let first = nums[0];
        let last = nums[nums.len() - 1];
        let num = 10 * first + last;
        sum += num;
    }
    Ok(format!("{}", sum))
}

fn part2(data: &str) -> Result<String> {
    let mut sum = 0;
    let re = Regex::new("(one|two|three|four|five|six|seven|eight|nine|[0-9])")?;
    for line in data.lines() {
        let mut nums = vec![];
        for i in 0..line.len() {
            if let Some(m) = re.find(&line[i..]) {
                nums.push(word_to_num(m.as_str()));
            }
        }
        let first = nums[0];
        let last = nums[nums.len() - 1];
        // eprintln!("line = {:#?}, {first}, {last}", line);
        let num = 10 * first + last;
        sum += num;
    }
    Ok(format!("{}", sum))
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
