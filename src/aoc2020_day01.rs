use anyhow::Result;

fn main() -> Result<()> {
    let data = std::fs::read_to_string("input/day1.txt")?;
    let tidy_data: Vec<&str> = data.split("\n").map(|x| x.trim()).collect();

    let lines: Vec<i64> = tidy_data
        .iter()
        .map(|x| x.parse())
        .filter_map(|x| x.ok())
        .collect();

    println!("AoC2020 1.1 - {}", part1(&lines));
    println!("AoC2020 1.2 - {}", part2(&lines));
    Ok(())
}

fn part1(lines: &[i64]) -> i64 {
    let mut result = 0;
    'outer: for (i, el_i) in lines.iter().enumerate() {
        for el_j in &lines[i..] {
            if el_i + el_j == 2020 {
                result = el_i * el_j;
                break 'outer;
            }
        }
    }
    result
}

fn part2(lines: &[i64]) -> i64 {
    let mut result = 0;
    'outer: for (i, el_i) in lines.iter().enumerate() {
        for el_j in &lines[i..] {
            for el_k in &lines[i + 1..] {
                if el_i + el_j + el_k == 2020 {
                    result = el_i * el_j * el_k;
                    break 'outer;
                }
            }
        }
    }
    result
}
