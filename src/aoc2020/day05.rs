use crate::part::Part;
use anyhow::Result;
use std::fmt::Display;

const ROW_LOWER_CHAR: char = 'F';
const COL_LOWER_CHAR: char = 'L';

pub fn day5(data: &[&str], part: Part) -> Result<()> {
    let mut seat_ids: Vec<i64> = data
        .iter()
        .filter(|x| x.len() == 10)
        .map(|&s| seat_id(seat_location(s)))
        .collect();
    seat_ids.sort();
    let id = match part {
        Part::One => seat_ids[seat_ids.len() - 1],
        Part::Two => seat_ids
            .iter()
            .zip(seat_ids[1..].iter())
            .filter(|(&a, b)| (*b - a) == 2)
            .map(|(a, _)| a + 1)
            .collect::<Vec<i64>>()[0],
    };
    println!("5.{} - {}", part, id);

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

#[allow(unused_imports)]
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
            let found_loc = seat_location(rule);
            let calc_id = seat_id(found_loc);
            assert_eq!(found_loc, seatloc);
            assert_eq!(calc_id, seatid);
        }
    }
}
