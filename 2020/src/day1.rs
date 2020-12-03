use super::Part;
use anyhow::Result;

pub fn day1(data: &[&str], part: Part) -> Result<()> {
    // Part 1
    // Find 2 entries that sum to 2020, and multiply them
    //
    // Part 2
    // Find 3 entries that sum to 2020, and multiply them
    let lines: Vec<i64> = data
        .iter()
        .map(|x| x.parse())
        .filter_map(|x| x.ok())
        .collect();
    let mut result = 0;
    'outer: for i in 0..lines.len() {
        let el_i = lines[i];
        for j in i..lines.len() {
            let el_j = lines[j];
            match part {
                Part::One => {
                    if el_i + el_j == 2020 {
                        result = el_i * el_j;
                        break 'outer;
                    }
                }
                Part::Two => {
                    for k in j..lines.len() {
                        let el_k = lines[k];
                        if el_i + el_j + el_k == 2020 {
                            result = el_i * el_j * el_k;
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    println!("1.{} - {}", part, result);
    Ok(())
}
