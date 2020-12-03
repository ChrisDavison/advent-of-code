#![allow(dead_code, unused_imports, unused_variables)]
use anyhow::Result;

#[derive(Clone, Copy)]
struct Slope {
    right: usize,
    down: usize,
}

struct Position {
    x: usize,
    y: usize,
}

fn next_position(current: Position, slope: Slope, width: usize) -> Position {
    let new_x = (current.x + slope.right) % width;
    let new_y = current.y + slope.down;
    Position { x: new_x, y: new_y }
}

pub fn run(data: &String, part: super::Part) -> Result<String> {
    // SAMPLE LINES
    // ..#..#......#..#.......#...#.#.
    // ...##.....##..#..#....#.##.##.#
    // ...#...#.##...##.....#.....#.#.
    let lines: Vec<String> = data.split("\n").map(|x| x.to_string()).collect();

    match part {
        super::Part::One => {
            let n_trees = check_slope(&lines, Slope { right: 3, down: 1 });
            Ok(format!("{}", n_trees))
        }
        super::Part::Two => {
            let slopes = vec![
                Slope { right: 1, down: 1 },
                Slope { right: 3, down: 1 },
                Slope { right: 5, down: 1 },
                Slope { right: 7, down: 1 },
                Slope { right: 1, down: 2 },
            ];
            let results: Vec<usize> = slopes.iter().map(|&s| check_slope(&lines, s)).collect();
            let mut product = results[0];
            for result in results.iter().skip(1) {
                product *= result;
            }
            Ok(format!("{}", product))
        }
    }
}

fn check_slope(lines: &[String], slope: Slope) -> usize {
    let mut current = Position { x: 0, y: 0 };
    let mut n_trees = 0;
    let width = lines[0].len();
    loop {
        current = next_position(current, slope, width);
        if current.y >= lines.len() {
            break;
        }
        if let Some(c) = lines[current.y].chars().nth(current.x) {
            if c == '#' {
                n_trees += 1;
            }
        }
    }
    n_trees
}
