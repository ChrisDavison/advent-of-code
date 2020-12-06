use anyhow::Result;

fn main() -> Result<()> {
    let data = std::fs::read_to_string("input/day3.txt")?;
    let tidy_data: Vec<&str> = data.split("\n").map(|x| x.trim()).collect();

    let slopes = vec![Slope { right: 3, down: 1 }];
    let slopes2 = vec![
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];

    let results: Vec<usize> = slopes.iter().map(|&s| check_slope(&tidy_data, s)).collect();
    let results2: Vec<usize> = slopes2
        .iter()
        .map(|&s| check_slope(&tidy_data, s))
        .collect();
    let product = |ls: &[usize]| -> usize { ls.iter().skip(1).fold(ls[0], |acc, x| acc * x) };

    println!("AoC2020 3.1 - {}", product(&results));
    println!("AoC2020 3.2 - {}", product(&results2));
    Ok(())
}

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
