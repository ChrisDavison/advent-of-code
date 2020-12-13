use anyhow::Result;
use std::fmt::{self, Display};

const CARDINALS: [char; 4] = ['N', 'E', 'S', 'W'];

pub fn day12(data: &str) -> Result<()> {
    let sample = include_str!("../samples/12-1.in");
    let sample2 = include_str!("../samples/12-2.in");
    let instructions_s: Vec<(char, usize)> = parse_instructions(sample);
    let instructions_s2: Vec<(char, usize)> = parse_instructions(sample2);
    let instructions: Vec<(char, usize)> = parse_instructions(data);

    println!("AoC2020 12.1 sample -> {}", part1(&instructions_s));
    println!("AoC2020 12.1 -> {}", part1(&instructions));
    println!("AoC2020 12.2 sample -> {}", part2(&instructions_s));
    println!("AoC2020 12.2 -> {}", part2(&instructions));

    Ok(())
}

fn parse_instructions(s: &str) -> Vec<(char, usize)> {
    s.lines()
        .filter_map(|row| Some((row.chars().nth(0).unwrap(), row[1..].parse().ok()?)))
        .collect()
}

fn part1(instructions: &[(char, usize)]) -> isize {
    let mut ship: Position = Position { north: 0, east: 0 };
    let mut facing = 'E';
    for &(letter, number) in instructions {
        if letter == 'L' || letter == 'R' {
            facing = rotate(facing, letter, number);
        } else {
            let letter = if letter == 'F' { facing } else { letter };
            move_by(&mut ship, letter, number);
        }
    }
    ship.manhattan()
}

fn part2(instructions: &[(char, usize)]) -> isize {
    let mut waypoint: Position = Position { north: 1, east: 10 };
    let mut ship: Position = Position { north: 0, east: 0 };
    for &(letter, number) in instructions {
        match letter {
            'L' | 'R' => rotate_waypoint(&mut waypoint, letter, number),
            'F' => {
                ship.north += waypoint.north * number as isize;
                ship.east += waypoint.east * number as isize;
            }
            _ => move_by(&mut waypoint, letter, number),
        }
    }
    ship.manhattan()
}

#[inline(always)]
fn move_by(current: &mut Position, direction: char, distance: usize) {
    match direction {
        'N' => current.north += distance as isize,
        'S' => current.north -= distance as isize,
        'E' => current.east += distance as isize,
        'W' => current.east -= distance as isize,
        _ => unreachable!("Cardinal can only be N, E, S, or W"),
    }
}

#[inline(always)]
fn rotate(starting: char, direction: char, angle: usize) -> char {
    let current = CARDINALS
        .iter()
        .position(|&x| x == starting)
        .expect("Cardinal must be N, E, S, or W");
    let delta = if direction == 'L' {
        -1 * (angle / 90) as isize
    } else {
        (angle / 90) as isize
    };

    let next = ((current as isize + delta) + 4) % 4;
    CARDINALS[next as usize]
}

#[inline(always)]
fn rotate_waypoint(current: &mut Position, direction: char, angle: usize) {
    let angle = if direction == 'L' { 360 - angle } else { angle };
    let (north, east) = match angle {
        90 => (-current.east, current.north),
        180 => (-current.north, -current.east),
        270 => (current.east, -current.north),
        _ => unreachable!("Direction L/R and angle 90/180/270"),
    };
    current.north = north;
    current.east = east;
}

#[derive(Clone, Copy)]
struct Position {
    north: isize,
    east: isize,
}

impl Position {
    fn manhattan(&self) -> isize {
        self.north.abs() + self.east.abs()
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.north, self.east)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.north, self.east)
    }
}
