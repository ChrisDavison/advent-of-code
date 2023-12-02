#![allow(dead_code, unused_variables)]
use aoc2023::*;

lazy_static! {
    static ref RE_CUBE: Regex = Regex::new(r"(\d+ red|\d+ green|\d+ blue)").unwrap();
}

pub fn day02() -> Result<String> {
    let data = parse(include_str!("../input/day02"));

    let output = format!(
        "2023 02.1 -> {}\n2023 02.2 -> {}",
        part1(&data)?,
        part2(&data)?
    );
    Ok(output)
}

#[derive(Debug, Clone, PartialEq)]
struct Game {
    id: usize,
    cubes: (i32, i32, i32),
}

fn parse_cube(s: &str) -> (&str, i32) {
    let (number, colour) = s.trim().split_once(' ').unwrap();
    (colour, number.parse().unwrap())
}

fn parse_cubeset(s: &str) -> Vec<(&str, i32)> {
    s.split(',').map(parse_cube).collect()
}

fn parse_line(line: &str) -> Game {
    // The game wants the MAXIMUM number of any particular cube seen
    // (the cubes get put into the back between each set within a game)
    let colon = line.find(':').unwrap();
    let (idstr, cubestr) = (
        &line[..colon].trim_start_matches("Game "),
        &line[colon + 2..],
    );
    let mut max_seen = HashMap::new();
    let mut max_seen_tuple = (0, 0, 0);
    for cubeset in cubestr.split(';').map(parse_cubeset) {
        for (colour, number) in cubeset {
            match colour {
                "red" => max_seen_tuple.0 = max_seen_tuple.0.max(number),
                "green" => max_seen_tuple.1 = max_seen_tuple.1.max(number),
                "blue" => max_seen_tuple.2 = max_seen_tuple.2.max(number),
                x => unreachable!("Unexpected colour {}", x),
            }
            let e = max_seen.entry(colour).or_insert(0);
            if number > *e {
                *e = number;
            }
        }
    }
    Game {
        id: idstr.parse().unwrap(),
        cubes: max_seen_tuple,
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}: R{} G{} B{}>",
            self.id, self.cubes.0, self.cubes.1, self.cubes.2
        )
    }
}

fn is_possible(game: &Game, limits: &(i32, i32, i32)) -> bool {
    game.cubes.0 <= limits.0 && game.cubes.1 <= limits.1 && game.cubes.2 <= limits.2
}

fn parse(data: &str) -> Vec<Game> {
    data.lines().map(parse_line).collect()
}

fn part1(games: &[Game]) -> Result<usize> {
    let limits = (12, 13, 14);
    Ok(games
        .iter()
        .filter(|g| is_possible(g, &limits))
        .map(|g| g.id)
        .sum())
}

fn part2(games: &[Game]) -> Result<i32> {
    Ok(games
        .iter()
        .map(|game| game.cubes.0 * game.cubes.1 * game.cubes.2)
        .sum())
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
        assert_eq!(part1(&parse(TEST_INPUT)).unwrap(), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(TEST_INPUT)).unwrap(), 2286);
    }
}
