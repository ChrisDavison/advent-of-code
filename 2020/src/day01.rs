use aoc2020::*;

const TARGET: i32 = 2020;

pub fn day01() -> Result<String> {
    let data = include_str!("../input/day01");
    let lines: HashSet<i32> = data
        .lines()
        .filter_map(|x| x.trim().parse().ok())
        .filter(|&x| x < TARGET)
        .collect();

    let output = format!(
        "2020 01.1 -> {}\n2020 01.2 -> {}",
        part1(&lines)?,
        part2(&lines)?
    );
    Ok(output)
}

fn part1(data: &HashSet<i32>) -> Result<String> {
    let product_of_pair = find_pair_sums_to(TARGET, data).map(|(a, b)| a * b);
    if let Some(prod) = product_of_pair {
        Ok(format!("{}", prod))
    } else {
        Err(anyhow!("Failed to find pair"))
    }
}

fn part2(data: &HashSet<i32>) -> Result<String> {
    let product_of_triple = find_triple_sums_to(TARGET, data).map(|(a, b, c)| a * b * c);
    if let Some(prod) = product_of_triple {
        Ok(format!("{}", prod))
    } else {
        Err(anyhow!("Failed to find triple"))
    }
}

fn find_pair_sums_to(target: i32, data: &HashSet<i32>) -> Option<(i32, i32)> {
    for num in data {
        let need = target - *num;
        if data.contains(&need) {
            return Some((*num, need));
        }
    }
    None
}

fn find_triple_sums_to(target: i32, data: &HashSet<i32>) -> Option<(i32, i32, i32)> {
    for num in data {
        let need = target - num;
        if let Some((a, b)) = find_pair_sums_to(need, data) {
            return Some((*num, a, b));
        }
    }
    None
}
