use anyhow::Result;
use itertools::Itertools;

const EMPTY: u8 = b'L';
const TAKEN: u8 = b'#';
const FLOOR: u8 = b'.';

const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn day11() -> Result<()> {
    let data = std::fs::read_to_string("input/11.in")?;
    let seats = data
        .split('\n')
        .map(|x| x.bytes().collect())
        .collect::<Vec<Vec<_>>>();
    part_1(&seats);
    part_2(&seats);
    Ok(())
}

fn part_1(seats: &[Vec<u8>]) -> usize {
    let result = iterate_till_stable(seats.to_vec(), &should_swap_p1);
    println!("2020 11.1 -> {}", result);
    result
}

fn part_2(seats: &[Vec<u8>]) -> usize {
    let result = iterate_till_stable(seats.to_vec(), &should_swap_p2);
    println!("2020 11.2 -> {}", result);
    result
}

fn find_neighbour(
    data: &[Vec<u8>],
    (i, j): (usize, usize),
    (di, dj): (isize, isize),
) -> Option<u8> {
    let (mut i, mut j) = (i as isize, j as isize);
    loop {
        i += di;
        j += dj;
        match data.get(i as usize).and_then(|row| row.get(j as usize)) {
            Some(&FLOOR) => {} // floor, so keep going
            Some(&c) => return Some(c),
            None => break, // Hit the wall
        }
    }
    None
}

fn should_swap_p1(data: &[Vec<u8>], i: usize, j: usize) -> bool {
    let mut neighbours = DIRS
        .iter()
        .map(|(di, dj)| (i as isize + di, j as isize + dj))
        .filter_map(|(i, j)| data.get(i as usize).and_then(|row| row.get(j as usize)));
    match data[i][j] {
        b'L' => neighbours.all(|&c| c != b'#'),
        b'#' => neighbours.filter(|&c| *c == b'#').count() >= 4,
        _ => false,
    }
}

fn should_swap_p2(data: &[Vec<u8>], i: usize, j: usize) -> bool {
    let mut neighbours = DIRS
        .iter()
        .filter_map(|&direc| find_neighbour(&data.to_vec(), (i, j), direc));
    match data[i][j] {
        EMPTY => neighbours.all(|c| c != TAKEN),
        TAKEN => neighbours.filter(|&c| c == TAKEN).count() >= 5,
        _ => unreachable!(),
    }
}

fn iterate_till_stable<F: Fn(&[Vec<u8>], usize, usize) -> bool>(
    data: Vec<Vec<u8>>,
    swap_checker: F,
) -> usize {
    let mut data = data;
    let mut indices_to_swap = Vec::new();
    loop {
        indices_to_swap.clear();
        for (i, j) in (0..data.len()).cartesian_product(0..data[0].len()) {
            if data[i].is_empty() {
                continue;
            }
            if data[i][j] != FLOOR && swap_checker(&data, i, j) {
                indices_to_swap.push((i, j));
            }
        }
        for &(i, j) in &indices_to_swap {
            data[i][j] = if data[i][j] == TAKEN { EMPTY } else { TAKEN };
        }
        if indices_to_swap.is_empty() {
            break;
        }
    }
    data.iter()
        .flat_map(|row| row.iter().filter(|&y| *y == TAKEN))
        .count()
}

#[allow(dead_code)]
fn bytegrid_to_string(grid: &[Vec<u8>]) -> String {
    grid.iter()
        .map(|x| String::from_utf8_lossy(x).to_string())
        .collect::<Vec<String>>()
        .join("\n")
}
