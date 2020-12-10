use anyhow::Result;
use rayon::prelude::*;

const DAY: usize = 5;

const ROW_LOWER_CHAR: char = 'F';
const ROW_UPPER_CHAR: char = 'B';
const COL_LOWER_CHAR: char = 'L';
const COL_UPPER_CHAR: char = 'R';

pub fn day05(data: &str) -> Result<()> {
    let tidy_data: Vec<&str> = data
        .as_parallel_string()
        .lines()
        .map(|x| x.trim())
        .collect();

    let seat_ids = sorted_seat_ids(&tidy_data);
    let id1 = seat_ids[seat_ids.len() - 1];
    let id2 = seat_ids
        .windows(2)
        .filter(|pair| (pair[1] - pair[0]) == 2)
        .map(|pair| pair[0] + 1)
        .collect::<Vec<i64>>()[0];
    println!("2020 {}-1 -> {}", DAY, id1);
    println!("2020 {}-2 -> {}", DAY, id2);

    Ok(())
}

fn sorted_seat_ids(data: &[&str]) -> Vec<i64> {
    let mut seat_ids: Vec<i64> = data
        .iter()
        .filter(|x| x.len() == 10)
        .map(|&s| seat_location(s))
        .map(|(row, col)| row * 8 + col)
        .collect();
    seat_ids.sort_unstable();
    seat_ids
}

fn seat_location(s: &str) -> (i64, i64) {
    let s: String = s
        .chars()
        .map(|c| match c {
            ROW_LOWER_CHAR | COL_LOWER_CHAR => '0',
            ROW_UPPER_CHAR | COL_UPPER_CHAR => '1',
            _ => '0',
        })
        .collect();
    let row = i64::from_str_radix(&s[..7], 2).unwrap();
    let col = i64::from_str_radix(&s[7..], 2).unwrap();
    (row, col)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seat_locations() {
        assert_eq!(seat_location("FBFBBFFRLR"), (44, 5));
        assert_eq!(seat_location("BFFFBBFRRR"), (70, 7));
        assert_eq!(seat_location("FFFBBBFRRR"), (14, 7));
        assert_eq!(seat_location("BBFFBBFRLL"), (102, 4));
    }

    #[test]
    fn examples_part1() {
        let inputs = vec!["FBFBBFFRLR", "BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"];
        let expected = 820;
        let sorted_ids = sorted_seat_ids(&inputs);
        assert_eq!(sorted_ids.last().unwrap(), &expected);
    }
}
