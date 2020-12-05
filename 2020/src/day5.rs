use crate::part::Part;
use anyhow::{bail, Result};
use std::fmt::{Debug, Display};

const ROW_LOWER_CHAR: char = 'F';
const COL_LOWER_CHAR: char = 'L';

pub fn day5(data: &[&str], part: Part) -> Result<()> {
    let seats: Vec<(i64, i64)> = data
        .iter()
        .filter(|x| x.len() == 10)
        .map(|&s| seat_location(s))
        .collect();
    let mut seat_and_id: Vec<(i64, i64, i64)> = seats
        .iter()
        .map(|&seat| (seat.0, seat.1, seat_id(seat)))
        .collect();
    match part {
        Part::One => {
            println!(
                "5.{} - {}",
                part,
                seat_and_id.iter().map(|(row, col, id)| id).max().unwrap()
            );
        }
        Part::Two => {
            seat_and_id.sort_by_key(|x| x.2);
            let my_seat_id: Vec<i64> = seat_and_id
                .iter()
                .cloned()
                .zip(seat_and_id[1..].iter())
                .filter(|(a, b)| (b.2 - a.2 == 2))
                .map(|(a, &b)| a.2 + 1)
                .collect();
            if my_seat_id.len() == 1 {
                println!("5.{} - {}", part, my_seat_id[0]);
            } else {
                bail!("Found multiple seat ids: {:#?}", my_seat_id)
            }
        }
    }

    Ok(())
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

fn binary_partition<T: Clone + Debug + Display, U: Eq>(
    values: &[T],
    directions: &[U],
    low_decider: U,
) -> T {
    if values.len() == 1 {
        values[0].clone()
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
            binary_partition(&input_127, &directions_ex1_rows, ROW_LOWER_CHAR),
            44
        );
        assert_eq!(
            binary_partition(&input_8, &directions_ex1_cols, COL_LOWER_CHAR),
            5
        );
    }

    #[test]
    fn test_day5_examples_part1() {
        let tests = vec![
            ("FBFBBFFRLR", (44, 5), 357),
            ("BFFFBBFRRR", (70, 7), 567),
            ("FFFBBBFRRR", (14, 7), 119),
            ("BBFFBBFRLL", (102, 4), 820),
        ];
        for (rule, seatloc, seatid) in tests {
            let found_loc = seat_location(rule).unwrap();
            let calc_id = seat_id(found_loc).unwrap();
            assert_eq!(found_loc, seatloc);
            assert_eq!(calc_id, seatid);
        }
    }
}
