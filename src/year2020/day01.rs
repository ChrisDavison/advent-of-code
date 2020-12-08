use anyhow::Result;

const DAY: usize = 1;

pub fn solve() -> Result<()> {
    let data = std::fs::read_to_string(format!("input/day{}.txt", DAY))?;
    let tidy_data: Vec<&str> = data.split('\n').map(|x| x.trim()).collect();

    let lines: Vec<i64> = tidy_data
        .iter()
        .map(|x| x.parse())
        .filter_map(|x| x.ok())
        .collect();

    println!("2020 {}-1 -> {}", DAY, part1(&lines));
    println!("2020 {}-2 -> {}", DAY, part2(&lines));
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
