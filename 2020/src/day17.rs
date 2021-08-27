use aoc2020::*;

pub fn day17() -> Result<String> {
    let input = include_str!("../input/day17");
    let data: Vec<Vec<u8>> = input
        .lines()
        .map(|x| x.chars().map(|y| if y == '#' { 1 } else { 0 }).collect())
        .collect();
    Ok(format!(
        "2020 17.1 -> {}\n2020 17.2 -> {}",
        part1(&data)?,
        part2(&data)?
    ))
}

/// Toggle rule
///
/// If a cube is active and exactly 2 or 3 of its neighbors are also active,
/// the cube remains active. Otherwise, the cube becomes inactive.
/// If a cube is inactive but exactly 3 of its neighbors are active, the cube
/// becomes active. Otherwise, the cube remains inactive.
fn should_be_active(cell: u8, n_neighbours_alive: u8) -> bool {
    if cell == 1u8 {
        n_neighbours_alive == 2 || n_neighbours_alive == 3
    } else {
        n_neighbours_alive == 3
    }
}

type Position = (usize, usize, usize);
type Position4d = (usize, usize, usize, usize);

/// Conway's game of life in 3d -- cellular automata
fn part1(data: &[Vec<u8>]) -> Result<usize> {
    let dirs_3d = (-1..=1)
        .cartesian_product(-1..=1)
        .cartesian_product(-1..=1)
        .map(|((i, j), k)| (i, j, k))
        .filter(|&(i, j, k)| (i, j, k) != (0, 0, 0))
        .collect::<Vec<(isize, isize, isize)>>();
    let n_iterations = 7;
    let padding = n_iterations;
    let max_width = data[0].len() + (2 * padding);
    let mut cube = create_cube(max_width, 0);
    for (rownum, row) in data.iter().enumerate() {
        for (colnum, state) in row.iter().enumerate() {
            cube[padding + 1][rownum + padding][colnum + padding] = *state;
        }
    }
    let indices = (0..max_width)
        .cartesian_product(0..max_width)
        .cartesian_product(0..max_width)
        .map(|((i, j), k)| (i as usize, j as usize, k as usize))
        .collect::<Vec<_>>();

    let mut active = HashSet::new();
    for _ in 0..n_iterations {
        active = indices
            .iter()
            .filter(|(i, j, k)| cube[*i][*j][*k] == 1)
            .collect::<HashSet<&Position>>();

        // use 'active' to generate HashMap<Position, n_active_neighbours>
        // iterate over 'active'
        //   calculate their neighbours, and add +1 to each of them
        let mut neighbours = HashMap::new();
        for &(i, j, k) in &active {
            for (di, dj, dk) in dirs_3d.iter() {
                let ii = delta_idx(*i, *di);
                let jj = delta_idx(*j, *dj);
                let kk = delta_idx(*k, *dk);
                *neighbours.entry((ii, jj, kk)).or_insert(0) += 1;
            }
        }

        indices.iter().for_each(|(i, j, k)| cube[*i][*j][*k] = 0);
        for ((i, j, k), count) in neighbours {
            //println!("({},{},{}) -> {}", i, j, k, count);
            let cell = if active.contains(&(i, j, k)) { 1 } else { 0 };
            if should_be_active(cell, count) {
                cube[i][j][k] = 1;
            }
        }
    }
    Ok(active.len())
}

/// Conway's game of life in 4d -- cellular automata
fn part2(data: &[Vec<u8>]) -> Result<usize> {
    let dirs_4d = (-1..=1)
        .cartesian_product(-1..=1)
        .cartesian_product(-1..=1)
        .cartesian_product(-1..=1)
        .map(|(((i, j), k), l)| (i, j, k, l))
        .filter(|&(i, j, k, l)| (i, j, k, l) != (0, 0, 0, 0))
        .collect::<Vec<(isize, isize, isize, isize)>>();
    let n_iterations = 7;
    let padding = n_iterations;
    let max_width = data[0].len() + (2 * padding);
    let mut hypercube = create_hypercube(max_width, 0);
    for (rownum, row) in data.iter().enumerate() {
        for (colnum, state) in row.iter().enumerate() {
            hypercube[padding + 1][padding + 1][rownum + padding][colnum + padding] = *state;
        }
    }
    let indices = (0..max_width)
        .cartesian_product(0..max_width)
        .cartesian_product(0..max_width)
        .cartesian_product(0..max_width)
        .map(|(((i, j), k), l)| (i as usize, j as usize, k as usize, l as usize))
        .collect::<Vec<_>>();

    let mut active = HashSet::new();
    for _ in 0..n_iterations {
        active = indices
            .iter()
            .filter(|(i, j, k, l)| hypercube[*i][*j][*k][*l] == 1)
            .collect::<HashSet<&Position4d>>();

        // use 'active' to generate HashMap<Position, n_active_neighbours>
        // iterate over 'active'
        //   calculate their neighbours, and add +1 to each of them
        let mut neighbours = HashMap::new();
        for &(i, j, k, l) in &active {
            for (di, dj, dk, dl) in dirs_4d.iter() {
                let ii = delta_idx(*i, *di);
                let jj = delta_idx(*j, *dj);
                let kk = delta_idx(*k, *dk);
                let ll = delta_idx(*l, *dl);
                *neighbours.entry((ii, jj, kk, ll)).or_insert(0) += 1;
            }
        }

        indices
            .iter()
            .for_each(|(i, j, k, l)| hypercube[*i][*j][*k][*l] = 0);
        for ((i, j, k, l), count) in neighbours {
            let cell = if active.contains(&(i, j, k, l)) { 1 } else { 0 };
            if should_be_active(cell, count) {
                hypercube[i][j][k][l] = 1;
            }
        }
    }
    Ok(active.len())
}

fn delta_idx(idx: usize, delta: isize) -> usize {
    (idx as isize + delta) as usize
}

fn create_cube(n: usize, value: u8) -> Vec<Vec<Vec<u8>>> {
    let row = repeat(value).take(n).collect();
    let col = repeat(row).take(n).collect();
    repeat(col).take(n).collect()
}
fn create_hypercube(n: usize, value: u8) -> Vec<Vec<Vec<Vec<u8>>>> {
    repeat(create_cube(n, value)).take(n).collect()
}

#[allow(dead_code)]
const SAMPLE: &str = ".#.
..#
###";
