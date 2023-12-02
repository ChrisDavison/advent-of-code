#![allow(dead_code, unused_variables)]
use aoc2023::*;

lazy_static! {
    static ref RE_CUBE: Regex = Regex::new(r"(\d+ red|\d+ green|\d+ blue)").unwrap();
}

pub fn day02() -> Result<String> {
    let data = include_str!("../input/day02");

    let output = format!(
        "2023 02.1 -> {}\n2023 02.2 -> {}",
        part1(data)?,
        part2(data)?
    );
    Ok(output)
}

#[derive(Debug, Clone, PartialEq)]
struct Game<'a> {
    id: usize,
    cubes: HashMap<&'a str, i32>,
    possible: bool,
}

fn parse_cube(s: &str) -> (&str, i32) {
    let (number, colour) = s.trim().split_once(' ').unwrap();
    (colour, number.parse().unwrap())
}

fn parse_cubeset(s: &str) -> Vec<(&str, i32)> {
    s.split(',').map(parse_cube).collect()
}

fn parse_line<'a>(line: &'a str, limits: &HashMap<&'a str, i32>) -> Game<'a> {
    // The game wants the MAXIMUM number of any particular cube seen
    // (the cubes get put into the back between each set within a game)
    let colon = line.find(':').unwrap();
    let (idstr, cubestr) = (
        &line[..colon].trim_start_matches("Game "),
        &line[colon + 2..],
    );
    let mut max_seen = HashMap::new();
    for cubeset in cubestr.split(';').map(parse_cubeset) {
        for (colour, number) in cubeset {
            let e = max_seen.entry(colour).or_insert(0);
            if number > *e {
                *e = number;
            }
        }
    }
    let valid = is_possible(&max_seen, limits);
    Game {
        id: idstr.parse().unwrap(),
        cubes: max_seen,
        possible: valid,
    }
}

impl<'a> std::fmt::Display for Game<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}: R{} G{} B{}>",
            self.id, self.cubes["red"], self.cubes["green"], self.cubes["blue"]
        )
    }
}

fn is_possible(cubes: &HashMap<&str, i32>, limits: &HashMap<&str, i32>) -> bool {
    for (&colour, &limit) in limits.iter() {
        if cubes.contains_key(colour) && cubes[&colour] > limit {
            return false;
        }
    }
    true
}

fn part1(data: &str) -> Result<usize> {
    let mut limits = HashMap::new();
    limits.insert("red", 12);
    limits.insert("green", 13);
    limits.insert("blue", 14);
    let mut sum_possible = 0;
    for line in data.lines() {
        let g = parse_line(line, &limits);
        if g.possible {
            sum_possible += g.id;
        }
    }
    Ok(sum_possible)
}

fn part2(data: &str) -> Result<i32> {
    let mut limits = HashMap::new();
    limits.insert("red", 12);
    limits.insert("green", 13);
    limits.insert("blue", 14);
    let mut prod = 0;
    for line in data.lines() {
        let g = parse_line(line, &limits);
        let power = g.cubes.values().product::<i32>();
        prod += power;
    }
    Ok(prod)
}

#[allow(dead_code)]
const TEST_INPUT: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_parse_cube() {
        assert_eq!(parse_cube("3 blue"), ("blue", 3));
        assert_eq!(parse_cube("6 red"), ("red", 6));
        assert_eq!(parse_cube("0 green"), ("green", 0));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 2286);
    }
}
