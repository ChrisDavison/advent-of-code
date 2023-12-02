use aoc2023::*;

pub fn day02() -> Result<String> {
    let data = parse(include_str!("../input/day02"));

    let output = format!(
        "2023 02.1 -> {}\n2023 02.2 -> {}",
        part1(&data)?,
        part2(&data)?
    );
    Ok(output)
}

impl Game {
    fn power(&self) -> i32 {
        self.cubes.0 * self.cubes.1 * self.cubes.2
    }

    fn is_possible(&self, limits: &Tuple3<i32>) -> bool {
        self.cubes.0 <= limits.0 && self.cubes.1 <= limits.1 && self.cubes.2 <= limits.2
    }
}

type Tuple3<T> = (T, T, T);

fn elementwise_max_assign<T: std::cmp::Ord + Copy>(a: &mut Tuple3<T>, b: &Tuple3<T>) {
    a.0 = a.0.max(b.0);
    a.1 = a.1.max(b.1);
    a.2 = a.2.max(b.2);
}

#[derive(Debug, Clone, PartialEq)]
struct Game {
    id: usize,
    cubes: Tuple3<i32>,
}

fn parse_cube_tuple(s: &str) -> Tuple3<i32> {
    let (number, colour) = s.trim().split_once(' ').unwrap();
    let number = number.parse().unwrap_or(0);
    match colour {
        "red" => (number, 0, 0),
        "green" => (0, number, 0),
        "blue" => (0, 0, number),
        x => unreachable!("Unexpected colour {}", x),
    }
    // (colour, number.parse().unwrap())
}

fn parse_cubeset_tuple(s: &str) -> Vec<Tuple3<i32>> {
    s.split(',').map(parse_cube_tuple).collect()
}

fn parse_line(line: &str) -> Game {
    // The game wants the MAXIMUM number of any particular cube seen
    // (the cubes get put into the back between each set within a game)
    let (idstr, cubestr) = line.split_once(':').expect("Unexpected line format");

    let mut max_seen_tuple = (0, 0, 0);
    for cubesets in cubestr.split(';').map(parse_cubeset_tuple) {
        for cubeset in cubesets {
            elementwise_max_assign(&mut max_seen_tuple, &cubeset);
        }
    }
    Game {
        id: idstr.trim_start_matches("Game ").parse().unwrap(),
        cubes: max_seen_tuple,
    }
}

fn parse(data: &str) -> Vec<Game> {
    data.lines().map(parse_line).collect()
}

fn part1(games: &[Game]) -> Result<usize> {
    let limits = (12, 13, 14);
    Ok(games
        .iter()
        .filter(|g| g.is_possible(&limits))
        .map(|g| g.id)
        .sum())
}

fn part2(games: &[Game]) -> Result<i32> {
    Ok(games.iter().map(|game| game.power()).sum())
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
    fn test_part1() {
        assert_eq!(part1(&parse(TEST_INPUT)).unwrap(), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(TEST_INPUT)).unwrap(), 2286);
    }
}
