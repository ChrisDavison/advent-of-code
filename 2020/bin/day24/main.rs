use aoc2020::*;

const NEIGHBOUR_DIRS: [(isize, isize); 6] = [
    (0, 1),  // east
    (0, -1), // west
    (-1, 1), // se
    (-1, 0), // sw
    (1, -1), // ne
    (1, 0),  // nw
];

const WIDTH: usize = 150;

fn main() -> Result<()> {
    let input = include_str!("input");
    let tiles: Vec<Hex> = parse_each(INPUT.lines());
    let renovated = get_starting_floor_state(&tiles);
    println!("2020 24.1 -> {}", part1(&renovated)?);
    println!("2020 24.2 -> {}", part2(&renovated)?);
    Ok(())
}

fn count_black(floor: &HashMap<Hex, bool>) -> usize {
    floor.values().filter(|v| **v).count()
}

fn count_black_2(floor: &[Vec<bool>]) -> usize {
    floor
        .iter()
        .map(|row| row.iter().filter(|v| **v).count())
        .sum()
}

fn part1(state: &HashMap<Hex, bool>) -> Result<String> {
    Ok(format!("{}", count_black(&state)))
}

fn part2(state: &HashMap<Hex, bool>) -> Result<String> {
    let mut hexes_to_flip: Vec<(usize, usize)> = vec![];

    let mut grid = vec![vec![false; WIDTH]; WIDTH];

    let offset = (WIDTH / 2) as isize;
    for (tile, val) in state.clone() {
        grid[(offset + tile.north) as usize][(offset + tile.east) as usize] = val;
    }

    for _ in 0..100 {
        hexes_to_flip.clear();

        for i in 0..WIDTH {
            for j in 0..WIDTH {
                if should_flip_2((i, j), &grid) {
                    hexes_to_flip.push((i, j));
                }
            }
        }

        for (i, j) in &hexes_to_flip {
            grid[*i][*j] = !grid[*i][*j];
        }
    }
    Ok(format!("{}", count_black_2(&grid)))
}

fn should_flip_2((i, j): (usize, usize), grid: &[Vec<bool>]) -> bool {
    let get = |(i, j), (di, dj)| {
        let i = (i as isize + di) as usize;
        let j = (j as isize + dj) as usize;
        grid.get(i).and_then(|row| row.get(j))
    };

    let n_black = NEIGHBOUR_DIRS
        .iter()
        .filter_map(|(n, e)| get((i, j), (n, e)))
        .filter(|is_black| **is_black)
        .count();

    if grid[i][j] {
        // TILE is black
        n_black == 0 || n_black > 2
    } else {
        n_black == 2
    }
}

fn get_starting_floor_state(coords: &[Hex]) -> HashMap<Hex, bool> {
    let mut history: HashMap<Hex, bool> = HashMap::new();
    for &hex in coords {
        let e = history.entry(hex).or_insert(false);
        *e = !*e;
    }
    history
}

#[derive(Debug, Clone, PartialEq, Copy)]
struct Hex {
    north: isize,
    east: isize,
}

impl std::fmt::Display for Hex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:4.01},{:5.01})", self.north, self.east)
    }
}

impl std::hash::Hash for Hex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let s = format!("{:.1},{:.1}", self.north, self.east);
        s.hash(state);
    }
}

impl Eq for Hex {}

impl std::str::FromStr for Hex {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut north = 0;
        let mut east = 0;
        let mut cs = s.chars();
        loop {
            let current = cs.next();
            match current {
                Some('s') => {
                    north -= 1;
                    if let Some('e') = cs.next() {
                        east += 1;
                    }
                }
                Some('n') => {
                    north += 1;
                    if let Some('w') = cs.next() {
                        east -= 1;
                    }
                }
                Some('e') => east += 1,
                Some('w') => east -= 1,
                None => break,
                x => unreachable!("Letter that shouldn't exist... {:?}", x),
            }
        }
        Ok(Hex { north, east })
    }
}

#[allow(dead_code)]
const SAMPLE: &str = r#"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"#;
