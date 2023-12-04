use aoc2023::*;

pub fn day02() -> Result<(String, String)> {
    let mut part1sum = 0;
    let mut part2sum = 0;

    let limits = (12, 13, 14);
    for (id, tuple) in parse_iter(include_str!("../input/day02")) {
        if is_possible(tuple, &limits) {
            part1sum += id;
        }
        part2sum += product(tuple);
    }

    Ok((part1sum.to_string(), part2sum.to_string()))
}

type Triple = (i32, i32, i32);

fn is_possible(tuple: Triple, limits: &Triple) -> bool {
    tuple.0 <= limits.0 && tuple.1 <= limits.1 && tuple.2 <= limits.2
}

fn product(tuple: Triple) -> i32 {
    tuple.0 * tuple.1 * tuple.2
}

fn elementwise_max_assign(a: &mut Triple, b: &Triple) {
    a.0 = a.0.max(b.0);
    a.1 = a.1.max(b.1);
    a.2 = a.2.max(b.2);
}

fn parse_cube_tuple(s: &str) -> Triple {
    let (number, colour) = s.trim().split_once(' ').unwrap();
    let number = number.parse().unwrap_or(0);
    match colour {
        "red" => (number, 0, 0),
        "green" => (0, number, 0),
        "blue" => (0, 0, number),
        x => unreachable!("Unexpected colour {}", x),
    }
}

fn parse_cubeset_tuple(s: &str) -> Vec<Triple> {
    s.split(',').map(parse_cube_tuple).collect()
}

fn parse_line(line: &str) -> (i32, Triple) {
    // The game wants the MAXIMUM number of any particular cube seen
    // (the cubes get put into the back between each set within a game)
    let (idstr, cubestr) = line.split_once(':').expect("Unexpected line format");

    let mut max_seen_tuple = (0, 0, 0);
    for game_round in cubestr.split(';').map(parse_cubeset_tuple) {
        for cubeset in game_round {
            elementwise_max_assign(&mut max_seen_tuple, &cubeset);
        }
    }
    (
        idstr.trim_start_matches("Game ").parse().unwrap(),
        max_seen_tuple,
    )
}

fn parse_iter(data: &str) -> impl Iterator<Item = (i32, (i32, i32, i32))> + '_ {
    data.lines().map(parse_line)
}

#[allow(dead_code)]
const TEST_INPUT: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
