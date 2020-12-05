use crate::part::Part;
use anyhow::{bail, Result};
use std::fmt::{Debug, Display};

const ROW_LOWER_CHAR: char = 'F';
const COL_LOWER_CHAR: char = 'L';

pub fn day5(data: &[&str], part: Part) -> Result<()> {
    match part {
        Part::One => unimplemented!(),
        Part::Two => unimplemented!(),
    }
}

fn seat_location(s: &str) -> Result<(i64, i64)> {
    let chars: Vec<char> = s.to_uppercase().chars().collect();

    let rows: Vec<_> = (0..128).map(i64::from).collect();
    let cols: Vec<_> = (0..8).map(i64::from).collect();
    let row = binary_partition(&rows, &chars[..7], ROW_LOWER_CHAR);
    let col = binary_partition(&cols, &chars[7..], COL_LOWER_CHAR);
    if row.len() == 1 && col.len() == 1 {
        Ok((row[0], col[0]))
    } else {
        bail!("Too many values returned")
    }
}

fn seat_id(s: &str) -> Result<i64> {
    let (row, col) = seat_location(s)?;
    Ok(row * 8 + col)
}

fn binary_partition<T: Clone + Debug + Display, U: Eq>(
    values: &[T],
    directions: &[U],
    low_decider: U,
) -> Vec<T> {
    println!("{}..{}", values[0], values[values.len() - 1],);
    if values.len() == 1 {
        values.to_vec()
    } else if directions.len() == 0 {
        values.to_vec()
    } else {
        let midpoint = (values.len() + 0) / 2;
        let new_values: &[T] = if directions[0] == low_decider {
            &values[0..midpoint]
        } else {
            &values[midpoint..]
        };
        binary_partition(&new_values, &directions[1..], low_decider)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_day5_binary_partition() {
        let input_127: Vec<i64> = (0..128).map(i64::from).collect();
        let input_8: Vec<i64> = (0..8).map(i64::from).collect();
        let directions_ex1_rows = ['F', 'B', 'F', 'B', 'B', 'F', 'F'];
        let directions_ex1_cols = ['R', 'L', 'R'];
        assert_eq!(
            binary_partition(&input_127, &directions_ex1_rows, ROW_LOWER_CHAR)[0],
            44
        );
        assert_eq!(
            binary_partition(&input_8, &directions_ex1_cols, COL_LOWER_CHAR)[0],
            5
        );
    }

    #[test]
    fn test_day5_examples_part1() {
        assert_eq!(seat_location("FBFBBFFRLR").unwrap(), (44, 5));
        assert_eq!(seat_id("FBFBBFFRLR").unwrap(), 357);

        assert_eq!(seat_location("BFFFBBFRRR").unwrap(), (70, 7));
        assert_eq!(seat_id("BFFFBBFRRR").unwrap(), 567);

        assert_eq!(seat_location("FFFBBBFRRR").unwrap(), (14, 7));
        assert_eq!(seat_id("FFFBBBFRRR").unwrap(), 119);

        assert_eq!(seat_location("BBFFBBFRLL").unwrap(), (102, 4));
        assert_eq!(seat_id("BBFFBBFRLL").unwrap(), 820);
    }
}
