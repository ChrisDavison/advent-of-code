use anyhow::Result;
use std::fmt::Display;

const ROW_LOWER_CHAR: char = 'F';
const COL_LOWER_CHAR: char = 'L';

fn main() -> Result<()> {
    let data = std::fs::read_to_string("input/day5.txt")?;
    let tidy_data: Vec<&str> = data.split("\n").map(|x| x.trim()).collect();

    let seat_ids = sorted_seat_ids(&tidy_data);
    let id1 = seat_ids[seat_ids.len() - 1];
    let id2 = seat_ids
        .iter()
        .zip(seat_ids[1..].iter())
        .filter(|(&a, b)| (*b - a) == 2)
        .map(|(a, _)| a + 1)
        .collect::<Vec<i64>>()[0];
    println!("AoC2020 5.1 - {}", id1);
    println!("AoC2020 5.2 - {}", id2);

    Ok(())
}

fn sorted_seat_ids(data: &[&str]) -> Vec<i64> {
    let mut seat_ids: Vec<i64> = data
        .iter()
        .filter(|x| x.len() == 10)
        .map(|&s| seat_id(seat_location(s)))
        .collect();
    seat_ids.sort();
    seat_ids
}

fn seat_location(s: &str) -> (i64, i64) {
    let chars: Vec<char> = s.to_uppercase().chars().collect();
    let rows: Vec<_> = (0..128).map(i64::from).collect();
    let cols: Vec<_> = (0..8).map(i64::from).collect();
    let row = binary_partition(&rows, &chars[..7], ROW_LOWER_CHAR);
    let col = binary_partition(&cols, &chars[7..], COL_LOWER_CHAR);
    (row, col)
}

fn seat_id(row_col: (i64, i64)) -> i64 {
    row_col.0 * 8 + row_col.1
}

fn binary_partition<T, U>(values: &[T], directions: &[U], low_decider: U) -> T
where
    T: Clone + Display,
    U: Copy + Eq,
{
    if values.len() == 1 {
        values[0].clone()
    } else {
        let midpoint = (values.len() + 0) / 2;
        let new_values = if directions[0] == low_decider {
            &values[0..midpoint]
        } else {
            &values[midpoint..]
        };
        binary_partition(&new_values, &directions[1..], low_decider)
    }
}

#[cfg(test)]
mod day5 {
    use super::*;

    #[test]
    fn binary_partition_rows() {
        let input_127: Vec<i64> = (0..128).map(i64::from).collect();
        let directions_ex1_rows = ['F', 'B', 'F', 'B', 'B', 'F', 'F'];
        assert_eq!(
            binary_partition(&input_127, &directions_ex1_rows, ROW_LOWER_CHAR),
            44
        );
    }

    #[test]
    fn binary_partition_cols() {
        let input_8: Vec<i64> = (0..8).map(i64::from).collect();
        let directions_ex1_cols = ['R', 'L', 'R'];
        assert_eq!(
            binary_partition(&input_8, &directions_ex1_cols, COL_LOWER_CHAR),
            5
        );
    }

    #[test]
    fn seat_locations() {
        assert_eq!(seat_location("FBFBBFFRLR"), (44, 5));
        assert_eq!(seat_location("BFFFBBFRRR"), (70, 7));
        assert_eq!(seat_location("FFFBBBFRRR"), (14, 7));
        assert_eq!(seat_location("BBFFBBFRLL"), (102, 4));
    }

    #[test]
    fn seat_ids() {
        assert_eq!(seat_id(seat_location("FBFBBFFRLR")), 357);
        assert_eq!(seat_id(seat_location("BFFFBBFRRR")), 567);
        assert_eq!(seat_id(seat_location("FFFBBBFRRR")), 119);
        assert_eq!(seat_id(seat_location("BBFFBBFRLL")), 820);
    }
    #[test]
    fn examples_part1() {
        let inputs = vec!["FBFBBFFRLR", "BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"];
        let expected = 820;
        let sorted_ids = sorted_seat_ids(&inputs);
        assert_eq!(sorted_ids.last().unwrap(), &expected);
    }
}
