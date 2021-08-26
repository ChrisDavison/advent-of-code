use aoc2020::*;

const MONSTER: &str = "..................#.#....##....##....###.#..#..#..#..#..#...";

pub fn day20() -> Result<()> {
    let input = include_str!("../input/day20");
    let tiles: Vec<Tile> = parse_each(input.split("\n\n"));
    let aligned = align_tiles(&tiles);

    println!("2020 20.1 -> {}", part1(&aligned));
    println!("2020 20.2 -> {}", part2(&aligned)?);

    Ok(())
}

fn part1(tile_grid: &[Vec<Tile>]) -> String {
    let n = tile_grid.len() - 1;
    let corner_product = [(0, 0), (0, n), (n, 0), (n, n)]
        .iter()
        .map(|&(x, y)| tile_grid[x][y].id)
        .product::<usize>();

    format!("{}", corner_product)
}

fn part2(tile_grid: &[Vec<Tile>]) -> Result<String> {
    let n = tile_grid[0].len();

    let img_height = tile_grid[0][0].size - 2;
    let mut img: Vec<String> = (0..n * img_height).map(|_| String::new()).collect();
    for (i, row) in tile_grid.iter().enumerate() {
        for tile in row {
            for (img_row, pixels) in tile.grid.iter().skip(1).take(img_height).enumerate() {
                let actual_col = img_row + img_height * i;
                img[actual_col].push_str(&pixels.iter().skip(1).take(img_height).join(""));
            }
        }
    }

    let img_str = format!("Tile 0:\n{}", img.join("\n"));

    let t: Tile = img_str.parse()?;

    let mut n_monster = 0;
    for orientation in t.orientations() {
        n_monster = count_monsters(&orientation);
        if n_monster > 0 {
            break;
        }
    }

    let n_hash_in_monster = n_monster * MONSTER.chars().filter(|c| *c == '#').count();
    let n_hash = format!("{}", t).replace(".", "").replace("\n", "").len();

    Ok(format!("{}", n_hash - n_hash_in_monster))
}

fn align_tiles(tiles: &[Tile]) -> Vec<Vec<Tile>> {
    let mut map: HashMap<usize, (isize, isize)> = HashMap::new();
    let mut tiles = tiles.to_vec();
    map.insert(tiles[0].id, (0, 0));

    loop {
        if map.len() == tiles.len() {
            break;
        }

        for i in 0..tiles.len() {
            let tile_a = tiles[i].clone();
            if !map.contains_key(&tile_a.id) {
                continue;
            }
            for (j, tile_b) in tiles.iter().enumerate() {
                if i == j {
                    continue;
                }
                if map.contains_key(&tile_b.id) {
                    continue;
                }

                let tilematch = tile_a.can_match(tile_b);
                if tilematch.is_none() {
                    continue;
                }
                let (t1, t2, direction) = tilematch.unwrap();
                let (dx, dy) = match direction {
                    Direction::North => (0, -1),
                    Direction::South => (0, 1),
                    Direction::East => (1, 0),
                    Direction::West => (-1, 0),
                };
                let mut new_pos = map[&t1.id];
                new_pos.0 += dx;
                new_pos.1 += dy;
                map.insert(t2.id, new_pos);
                tiles[j] = t2;
                break;
            }
        }
    }

    let n = (tiles.len() as f64).sqrt() as usize;
    let mut canvas: Vec<Vec<Tile>> = vec![vec![Default::default(); n]; n];

    let left = map.values().map(|(x, _y)| x).min().unwrap();
    let top = map.values().map(|(_x, y)| y).min().unwrap();

    // normalise all grid positions
    for (tile_id, position) in &map {
        let x = position.0 - left;
        let y = position.1 - top;
        canvas[y as usize][x as usize] = tiles.iter().find(|t| t.id == *tile_id).unwrap().clone();
    }
    canvas
}

fn count_monsters(t: &Tile) -> usize {
    let monster_idx: Vec<usize> = MONSTER
        .char_indices()
        .filter(|(_i, c)| *c == '#')
        .map(|(i, _)| i)
        .collect();

    let mut matches = 0;
    for stride in t.strides(20, 3) {
        let s = stride.iter().flatten().copied().collect::<Vec<char>>();
        if monster_idx.iter().all(|i| s[*i] == '#') {
            matches += 1;
        }
    }
    matches
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Default)]
struct Tile {
    id: usize,
    size: usize,
    grid: Vec<Vec<char>>,
}

impl std::str::FromStr for Tile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s
            .lines()
            .next()
            .map(|s| s.trim_end_matches(':'))
            .map(|s| s.split(' ').nth(1).expect("No tile ID"))
            .expect("No tile ID")
            .parse::<usize>()?;

        let grid: Vec<&str> = s.lines().skip(1).collect();
        let grid: Vec<Vec<char>> = grid.iter().map(|c| c.chars().collect()).collect();

        let size: usize = grid[0].len();
        Ok(Tile { id, size, grid })
    }
}

struct TileOrientations {
    t: Tile,
    iteration: usize,
}

impl std::iter::Iterator for TileOrientations {
    type Item = Tile;
    fn next(&mut self) -> Option<Self::Item> {
        if self.iteration > 8 {
            return None;
        }
        let mut t = if self.iteration < 4 {
            self.t.clone()
        } else {
            let mut t = self.t.clone();
            t.flip_top_bottom();
            t
        };
        // Don't need to do EW, as EW is just NS flip with rotation
        t.rotate_n(self.iteration % 4);

        self.iteration += 1;
        Some(t)
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Tile {
    fn rotate(&mut self) {
        let mut grid2 = self.grid.clone();

        for (i, row) in self.grid.iter().enumerate() {
            for (j, ch) in row.iter().enumerate() {
                grid2[j][self.size - i - 1] = *ch;
            }
        }
        self.grid = grid2;
    }

    fn rotate_n(&mut self, n: usize) {
        let n = n % 4;
        for _ in 0..n {
            self.rotate();
        }
    }

    fn flip_top_bottom(&mut self) {
        let mut grid2 = self.grid.clone();
        for (i, row) in self.grid.iter().enumerate() {
            grid2[self.size - i - 1] = row.clone();
        }
        self.grid = grid2;
    }

    fn orientations(&self) -> TileOrientations {
        TileOrientations {
            t: self.clone(),
            iteration: 0,
        }
    }

    fn can_match(&self, other: &Tile) -> Option<(Tile, Tile, Direction)> {
        // for orientation in self.orientations() {
        for orientation_2 in other.orientations() {
            if let Some(dir) = Tile::matching_edge(self, &orientation_2) {
                return Some((self.clone(), orientation_2, dir));
            }
        }
        // }
        None
    }

    fn edges(&self) -> (Vec<char>, Vec<char>, Vec<char>, Vec<char>) {
        let top = self.grid[0].clone();
        let bot = self.grid[self.size - 1].clone();
        let left = self.grid.iter().map(|row| row[0]).collect();
        let right = self.grid.iter().map(|row| row[self.size - 1]).collect();
        (top, right, bot, left)
    }

    fn matching_edge(a: &Tile, b: &Tile) -> Option<Direction> {
        let a = a.edges();
        let b = b.edges();
        if a.0 == b.2 {
            // North edge of A matches bot of B
            Some(Direction::North)
        } else if a.1 == b.3 {
            // East edge of A matches West of B
            Some(Direction::East)
        } else if a.2 == b.0 {
            // South edge of A matches North of B
            Some(Direction::South)
        } else if a.3 == b.1 {
            // West edge of A matches East of B
            Some(Direction::West)
        } else {
            None
        }
    }

    fn strides(&self, width: usize, height: usize) -> Strides<char> {
        Strides::new(&self.grid, width, height)
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let gridstr = self
            .grid
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", gridstr)
    }
}

#[allow(dead_code)]
const SAMPLE: &str = r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#;

#[allow(dead_code)]
const SAMPLE2: &str = r"Tile 1:
1abc
.,:;
wxyz
.,:;
";
