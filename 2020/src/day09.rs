use aoc2020::*;

pub fn day09() -> Result<String> {
    let data = include_str!("../input/day09");
    let lines: Vec<i64> = parse_each(data.lines());
    let window = 25;
    let output = format!(
        "2020 09.1 -> {}\n2020 09.2 -> {}",
        part1(&lines, window)?,
        part2(&lines, window)?
    );
    Ok(output)
}

fn part1(lines: &[i64], window: usize) -> Result<String> {
    let mut idx = window; // Start from an index that will fill buffer
    let mut found: Option<i64> = None;
    while let Some(n) = lines.get(idx) {
        let history = &lines[idx - window..idx];
        let mut found_a_pair = false;
        for (i, val) in history.iter().enumerate() {
            let need = n - val;
            if history[i..].contains(&need) {
                found_a_pair = true;
                break;
            }
        }
        if !found_a_pair {
            found = Some(*n);
            break;
        }
        idx += 1;
    }
    found
        .map(|x| format!("{}", x))
        .ok_or_else(|| anyhow!("No number found."))
}

fn part2(lines: &[i64], window: usize) -> Result<String> {
    let target: i64 = part1(lines, window)?.parse()?;

    let mut output = None;
    'outer: for window_size in 2..lines.len() - 1 {
        for idx in 0..lines.len() - window_size {
            let mut buffer: Vec<i64> = lines[idx..idx + window_size].to_vec();
            buffer.sort_unstable();
            if buffer.iter().sum::<i64>() == target {
                output = Some(buffer[0] + buffer[window_size - 1]);
                break 'outer;
            }
        }
    }
    output
        .map(|x| format!("{}", x))
        .ok_or_else(|| anyhow!("No num found"))
}

#[test]
fn p1_example() {
    let lines: Vec<i64> = SAMPLE
        .split("\n")
        .filter_map(|x| Some(x.parse::<i64>().ok()?))
        .collect();

    let window = 5;
    assert_eq!(part1(&lines, window).unwrap(), "127");
}

#[test]
fn p2_example() {
    let lines: Vec<i64> = SAMPLE
        .split("\n")
        .filter_map(|x| Some(x.parse::<i64>().ok()?))
        .collect();

    let window = 5;
    assert_eq!(part2(&lines, window).unwrap(), "62");
}

#[allow(dead_code)]
const SAMPLE: &str =
    "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";
