use aoc2020::*;

pub fn day03() -> Result<String> {
    let input = include_str!("../input/day03");
    let data: Vec<Vec<char>> = input.lines().map(|x| x.trim().chars().collect()).collect();

    let is_ok = check_slope(&data, (3, 1));
    let product = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&s| check_slope(&data, s))
        .product::<usize>();

    let output = format!("2020 03.1 -> {}\n2020 03.2 -> {}", is_ok, product);
    Ok(output)
}

fn check_slope(lines: &[Vec<char>], (dx, dy): (usize, usize)) -> usize {
    (0..lines.len() / dy + 1)
        .map(|i| (i * dx % lines[0].len(), i * dy))
        .filter(|(col, row)| lines.get(*row).and_then(|y| y.get(*col)) == Some(&'#'))
        .count()
}
