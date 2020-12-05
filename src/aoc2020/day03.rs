use crate::part::Part;
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

pub fn day3(data: &[&str], part: Part) -> Result<()> {
    let slopes = match part {
        Part::One => vec![Slope { right: 3, down: 1 }],
        Part::Two => vec![
            Slope { right: 1, down: 1 },
            Slope { right: 3, down: 1 },
            Slope { right: 5, down: 1 },
            Slope { right: 7, down: 1 },
            Slope { right: 1, down: 2 },
        ],
    };

    let results: Vec<usize> = slopes.iter().map(|&s| check_slope(&data, s)).collect();

    println!(
        "3.{} - {}",
        part,
        results.iter().skip(1).fold(results[0], |acc, x| acc * x)
    );
    Ok(())
}

fn next_position(current: Position, slope: Slope, width: usize) -> Position {
    let new_x = (current.x + slope.right) % width;
    let new_y = current.y + slope.down;
    Position { x: new_x, y: new_y }
}

fn check_slope(lines: &[&str], slope: Slope) -> usize {
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
