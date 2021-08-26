use aoc2020::*;

const EMPTY: u8 = b'L';
const TAKEN: u8 = b'#';
const FLOOR: u8 = b'.';

pub fn day11() -> Result<()> {
    let input = include_str!("../input/day11");
    let seats = input
        .lines()
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

type Position = (usize, usize);

fn should_swap_p1(data: &[Vec<u8>], pos: Position, directions: &[(isize, isize)]) -> bool {
    let mut neighbours = directions
        .iter()
        .map(|(di, dj)| (pos.0 as isize + di, pos.1 as isize + dj))
        .filter_map(|(i, j)| data.get(i as usize).and_then(|row| row.get(j as usize)));
    match data[pos.0][pos.1] {
        EMPTY => neighbours.all(|&c| c != TAKEN),
        TAKEN => neighbours.filter(|&c| *c == TAKEN).count() >= 4,
        _ => unreachable!(),
    }
}

fn should_swap_p2(data: &[Vec<u8>], pos: Position, directions: &[(isize, isize)]) -> bool {
    // (finished?, char found)
    let mut finished_and_found: Vec<(bool, u8)> = std::iter::repeat((false, b' '))
        .take(directions.len())
        .collect();

    let (ix, iy) = (pos.0 as isize, pos.1 as isize);
    let mut iter = 0;
    loop {
        for (i, (x, y)) in directions.iter().enumerate().take(8) {
            let mut p = finished_and_found.get_mut(i).unwrap();
            let x = ix + (iter + 1) * x;
            let y = iy + (iter + 1) * y;

            if !p.0 {
                let (fnew, c) = match data.get(x as usize).and_then(|row| row.get(y as usize)) {
                    Some(&FLOOR) => (false, FLOOR),
                    Some(&c) => (true, c),
                    None => (true, EMPTY), // Hit the wall
                };
                p.0 = fnew;
                p.1 = c;
            }
        }
        if finished_and_found.iter().all(|(finished, _)| *finished) {
            break;
        }
        iter += 1;
    }

    match data[pos.0][pos.1] {
        EMPTY => finished_and_found.iter().all(|(_, c)| *c != TAKEN),
        TAKEN => {
            finished_and_found
                .iter()
                .filter(|(_, c)| *c == TAKEN)
                .count()
                >= 5
        }
        _ => unreachable!(),
    }
}

fn iterate_till_stable<F: Fn(&[Vec<u8>], Position, &[(isize, isize)]) -> bool>(
    data: Vec<Vec<u8>>,
    swap_checker: F,
) -> usize {
    let mut data = data;
    let mut indices_to_swap = Vec::new();
    let neighbours: Vec<(isize, isize)> = (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|(i, j)| !(*i == 0 && *j == 0))
        .collect();
    let indices: Vec<(usize, usize)> = (0..data.len())
        .cartesian_product(0..data[0].len())
        .collect();

    loop {
        indices_to_swap.clear();
        for pos in &indices {
            if data[pos.0].is_empty() {
                continue;
            }
            if data[pos.0][pos.1] != FLOOR && swap_checker(&data, *pos, &neighbours) {
                indices_to_swap.push(pos);
            }
        }
        for &pos in &indices_to_swap {
            data[pos.0][pos.1] = if data[pos.0][pos.1] == TAKEN {
                EMPTY
            } else {
                TAKEN
            };
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
const SAMPLES: [&str; 3] = [
    "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
    "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
    "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##",
];
