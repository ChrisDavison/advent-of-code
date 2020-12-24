use aoc::prelude::*;

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
    let now = Instant::now();
    let tiles: Vec<Hex> = parse_each(INPUT.lines());
    let renovated = get_starting_floor_state(&tiles);
    let p1 = part1(&renovated)?;
    let after_1 = Instant::now();
    println!("2020 24.1 -> {} ({}ms)", p1, as_ms(after_1 - now));
    let p2 = part2(&renovated)?;
    let after_2 = Instant::now();
    println!("2020 24.2 -> {} ({}ms)", p2, as_ms(after_2 - after_1));

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
const INPUT: &str = r#"neenwnenwwnwnwnenwsesenwnwsenwnenwwnw
newnwnenenwnwneeneeswnwnwnenwneswsenwne
wenwswnwswwnenwnwnwwnwswenwneswnww
seneeswwneneneeneeneswneneneeswnee
swseswswneseswwenwneseswswswswnwsesesw
senwnwnwwwseneseenwnwsenwnwnenwnenwnw
seeneeseseseeeesesenweweseswesesese
nwwwnwnwwsenewsenwnwnwwwwwswew
swnwwsewnewwwsenenwnwwweweswwne
sweseseweseeesesenwenwseswsesenwewsw
neswwnwswnenenwsewwwnweneseenenwnw
eweeenweesweenweeeswneeeene
wswswswswweswsww
senesenwsenweneseneswwsenwswsewwsene
wnenwneenwneweeseneseneeseeneneenee
neeeeeeseseeeeeeewewneeeee
nwsewnwswnwnewsewwnwnweswwwnwwew
wnwnenesenenenwsenw
nenenenenewnenenenenwnenwnenenwnesenenese
senwwwswnwweewseswnewwewenwse
seswseseswswswswswnwswswseswswenwsesesw
eseneneneswnenewnenenenewsenenenwnene
nenwneneneneseenenenewsene
eseeenweeeneeeeeneee
nesesewseseneseseseseseswwsesewseesesee
swneseeseseseweneseseneneweseseswse
nenwneneneneenenenwneneneswsenw
eseesesewseseesenenwenwseseesewse
neweswswnwsenwwnesenwewseneseneseww
nenwneseweneseenenewnenewnenenewsw
senenwnwswnwnenwnwnwewnenwneenwnenwnw
wnwnwneseenewnwswwnwwewwsweww
esweneweewnewnewesweeeseeesene
nwnwnenenewnenenenenesenw
swwwswswswnewseenwwwseswswswwswwsw
seswswsesenwswswswnesweswswwswseswneswsw
swewswwneswswswwswswsw
wwseseneswewwswnwswwwnwwswswswnee
wnwwwwswnwnwnewnwwwnwwweenwsw
seseseswswswwswseseseseswneseswnesesew
seeseeseeswsenwnewseswswsewsewseswse
wweneeeneeneenenewneneneswsenene
enwwsewneneseeneneewesenwswwnwsw
nwwwnwnwnwnwnwwsenwwenwwwnwsewne
swswswseswswseswnwswnwsw
wswseneewswesenwwswnwswswwswnewswsw
seeeesenewnweeeseeesweenwseenee
seswnweesesewseseseswsenesesewsesenese
swneeneseenwnenwswenenwwseswswesenwe
swswnwsesenenwsenwsesesesesewswswsesenesw
swsesenweewsesesesesese
nwneeeeeseeeeesewseeseswwee
weeweeeenweswseeeeeseeee
ewwwwwnwwww
enwsweneeneeneseneneneeneswnewneese
wwswsewwwnwnwwnwnwsenwnwnwnwnwwne
seswsesweswwseswswwnwnwenenenwsenwnew
swwewnwswneswwseswwww
neneenewneneneneeeneneseweeneenee
nweneneeneeeeneneseneswneneenewnene
nesesenweneneswswswswwswswwenenwsww
swseeseseneseseseseswesesesesesesenwsese
senenewewnenenenewneswneene
nwnenenwnenesenesewsewnwsenewnwnwnee
nenenesenwwnenenenenwnenenenwne
seesenesesesesesesesesesesewseswsewse
swsenewswseswnwsewnesewwneseese
senwwnwnwnwwnwsenwwnwnwsenenenwswewnw
wseeswnwnwswnwnenwswseenwnewnwnwnew
nwwnwswnenwnwnwnwnwnwnwnweswnwwwnwnwnw
weswnwswweewneswwwwswwswnwsww
nwswswsenwnwnewwswsesweneewwneswe
nwwnwnwswnwenwnwwwswwwwwewwnwnww
nenenwnweneswneneseswenwneneswnewwneswne
swnenewneewneneneese
seeeeneeeseeenewnwwesweewse
neswsenewswswswsewswnesesewswswswswsw
nwnwwneneeswnwnwnwnwnwnwswnwnwnwwnwswnw
eswwnwnwnwenenwnwneswnenwnwnenwnenwnw
swnenenenwnenwnenewsweneneneneneneene
seseseseseswseseswsesesenwseenwseseswsese
eneseeswneneseeswnwsewwewenwnenew
eeenwnwsenwseesenweeweesweese
wnweseeseneseeeseseeswnwseseswsese
nwwswwseswswwswwwseneswwneswwwse
nwwseswneeseeneewsesesesewesesesese
wewwswseneswnwswswswwwswswnwswsew
nwnwnwnwnenwnenwnwnwnwenwnwnwnwnwneswswnw
nwweeeseseseenwneswswnwsenwnewswnwse
wnewwsenewwnwswnwswsesweeweswsw
nwneenwnwnwnewnwnwnwneswnwnwneswnenwnw
wswwswwswswswnwnwswwswweswwswew
nwsesenwsenwseseseseesewenwsewswseswe
swseseseseswwseseseseswswnewesw
nesenenwsenwesenwsesewneswseswsesesese
wwwswsewwnwewswwswnwwwswweneww
swswswswswswswswwswwswswswwneswsw
enwwwnwwwwwe
neneneeneneesenenwneswnenewnwnwneswnwne
swswnwswswneswnwswswswsweswswswswswswswse
nenwneneneeneneneswnewenewsenenewe
wswseenwnwwnwwnwwwwwnwwswenwne
swswswesenwseswswswwswwseswswnwsweswsw
wwwnewsenwwnwnwsewnwnwwnwnwnwsenewnw
eseseseseneesesesenesewseseseseseesw
wenwnwwnwwwwswewwwwenwwww
swenwnenenenwnwenewnwenwsweswneswnww
wwnwseweswsenwsesweseswseewswsenenesw
wnewwwwwwwwsewswsewswwwnesw
seeeeneneeneewe
eesesewseswswwwnesesenenesenwsesesw
wneswwnwnwnwnwenwnwswwwwwnwwnwwnw
seseseseeesenwswwsesesesesewe
nwnwnwwsewswwnwwwnwwnwenwenwnwwwnw
sewsenwwseswenwsenwsesenenesenwseseneswse
eeseeweseeseseeweseeneeeee
nwnwnwsenwnwnwwnenenwnenwswsewsenwnww
newnenenenenenenenenenewneeneseneneneswne
nwseswneesenwnwnwnwnesenwnwswneseneenw
wwseneneswwnesenwsesewwneneswwnww
wseesesenwseseseseesewsesenwseseesee
swwswwwwnwswwwwwsewwsewneswswne
swswsewswwwwwswwswnewsewwwwswne
nwnwnwnwswnenwnenenwnwnwnwnwnwnw
swseswswneswswswwswseswswswseswswswnesw
eeneneeneswneneneswneeneeenenenenwne
neswswswnwwwswwnwswswswwswweeswswsw
nwwwwswwenewswswwwnwnweswnweww
swsenewwswesewnwwnewneewwnewwse
wsenenwnwsenwwnwsewwnwwnwwneenw
swenenwnwnenwnwnenewnenenw
nwwenwwnwnwnwwnwswnwnwnenwnwwsewne
eseswsewseneswsenw
nwneeneneseenenenwwnwseswneneeneswnee
eneeeeeeswseeneweeenwseeenenee
nenwnenwnwnewneneneenwneswnenenenenene
nesenwwswsewnwnwewnwwswwnwwnwwenw
seesewseswswwseswnwwswseswseeesee
swwwwnwwswwweswswswsw
nenwnwnwnwnwnwnwnwnwnwsesenwnwenwswnwnwnw
swneneswnenenenenenenwsenwnenewnenwnwnwse
wneweeeeeeseeeseseseseseesesee
newnwneeewenwweswseseneswneeenwswe
seseseseeswswsesenesenewseswnenwwsenenw
wswwwnwswswsweewwwwswwwswswsw
swnenwnwneneseeeenenwwnenenenewnenwesw
nwwneneswseswnenwenwswnwnesewwsenwnwnwsw
swnenwswewsenwseewwnewswswnewwww
sesesesewesesesesenwneseseswseesesesese
wnwewwnewwwseswwwwswnewwww
swnwweeeneeeneneeee
swnenwneseswwswnwseneeswwnwwnenwnww
nenewnenenenewneneeenenwneneneenesw
enwenweswnenewesenwesweseeseweese
swewseswsweswswswswsewseseswseseswnesw
nesweenenwnweeneneneneseeneneeenewsw
nwnwnwnwwsenwnwnwnwnwnwnwnwnwnwnwnwnwe
swswswswswnwswesweswswswswnenwswswseswse
swsenenewwwnwwswnewwwseswswwswenese
nwsenwnwewwewwnwnwwnenwewnwnwse
swnenwnwwwswewnwnwseene
wwnwnwsenwnwnewnesenwnesenwwsewnwnww
nenenenenewneeneewseenwswneneneenee
senwseseseseseseswswseeseseswsese
nwnesenwnwnenewneneeswnenenwnenenenwnesw
seswwnenwenwwenwnwnwnwwwneswwwesew
neneeswwnenenenenenenenenenenenenewe
seseseseeswsenwswseseswseseswseneswswsw
enwnwsewsewneeeenenwseneeeneseene
nwewnwnenenenewneseenwnenenenenenewne
nwswneneneenenenesenwnenwnenenenenewnene
esweswnwswwswswwwwswswwswswesww
wwenesenwsenwwwnwnwswnwenwnwnwnwnw
seswwseseswseseseseswnwseneseseseswsesesw
eenewnwnwnwnenwnenenenwwnwnwnwnw
swwnwswswneneeswwswwwswwswswswsenese
nwswseseeseeseseseseseneeeneweesee
wswewswwswswnewswseswswewswswnwsene
nwneswnwnwneeneneneneneswnwnwnwnwnwnwnw
wwwwwwwewsewsenwnwwwwwwww
eewseseweseswsesesenwseneenwee
esweeneeeeweeeneneenee
eeeseenwseeseeeeswseeeneeese
eneneneneneeeeeeneneeeeneseesww
nweeeeneswenweeeeneneeneneswe
ewwnwsewwwwwwswnwnew
neseswswswseswseseswsewsenwsweseswswnwse
wwwwewwwnewwnwewsewsww
wwwswswnewwswwwswswsenwwswseswww
nwsesenwswsenwseswse
neneswenenenenwneneenewseneneneneene
wnweneeseeeeseeseseeseswneseesese
enewneesweeeeeseweeeeseenwnw
senwwnesweeseeeeeeeweneseseee
swwwwswwwswwswswwswneswswsw
seseseseseseneesesesesewse
swnwswsweenwswseswseswswswswswswswseswswse
nwwneeenwenenwneneswwsewseswnenwnwswne
nwwsenwsenweswswneeswswsw
swwswswswwswwewnewswswwswwswswsw
wwswwswswsenwwsw
seeeseseesewseeeeseewseesesese
eneeswweeeeeeeneesenwneseswnwesw
wnesewnwwnwnwsesenewwwnwenwnwnwwnw
swswswswwweswwwwwswswswwww
swsenwseneswnenewswswseneseswsee
seeswseseeseswswseswswswsesesesenwsesenwse
swweswwnwseswwnewswenewnesewwswnw
eenwseswnwseseseweseenesesesesesesesee
wwneeswwwwwsewwwwwwewwwww
wswseseswswseswswneswseswswneswsesenwwsee
seewwwwswswwwnwnwneswswsw
ewewswweesesesenwewesenwnwesese
nwnwnwnwewnwwnenwnwnwenwnwswnwsenwnw
newneeeneenewneneneenesenenwwenene
neeesweseeenwseseeseesesweswnwwse
wwwwwwswnewnwsewwwnwwwnwsewnw
nenenenesweneneneneneneenwnenewnwnene
enewsenenesenenenenenwneneneesesenwwne
nwnwenwnwnwneweewnwwsweenwsewne
wnwnwnwswwenwnwswewwnwnwnwnwnwnwnwwnw
nwneseenwnenwnenwswnwnwwnewneenenwenw
nweeweeewwnwneseenwswseseswnwsese
wswewswwwewnwnww
seswwsesenewwswnesesesweesesesesese
sweeenwnweeeesweeeeweswenw
nwnwnwnwsenwnwswnwswnwwnwnwnwnwsenenwe
wwewwswwwneswnwsewswwwwwswsw
swwwwwwwwwewnwwwwwwnw
sewwwswwwwwnwnwweswwwswwwwesw
nwwseneesenewswnwnenewwwesenwwnesene
nwswswnenenwnwnwnenenesenwswwnenwnenenw
nwnwnwnwnenwnwnwnewsenwnwnwnwnw
swswwseswswseswswsweseswswseseswsw
esesenweeeesewseseseseesenweswese
nwnwwswwwnwesewwneweswswnewnewww
swseseswneseeswsenwseseswseseswnwsesese
neneseseneenwswnenenwenenwnwswswnwnenwnw
nwnenenwsweneswnwnewswneneneswnenwnesese
nwsewnweenwnwneenwnwwsenwwsewnwnw
nwswwenwneneneneenwnwnwnesenenwnenwnw
seswnenenwnwwwnenweseswewnwwsesenw
neneseneneswneswnenenenwnwnwswnewnenenw
seesweeseeeseeeenwsesese
nwnenwenwnwnwnwnewnenwnwnwneswswnenw
seeeeeeeneeneseeweeeseeseewe
enenwwswnwnwnwwnenwwwsenwnwsenwnwsenwnw
wesenesewseseewse
nenesenenenenenwnenenewnenenwnwnesenenenw
neswnwswswwswswswneewnwweswnwwwwe
esewsenwweseseeseeweeewnewne
nwnwnenwwnwwnwswnenenenenesenwnwnenenwe
swseswnwswwwswswnenwnwswswesesewsenwnw
wesenwenwewwwwswnwneswwnenenwnw
seseseseseswsesesenwswseneseseseseseswwse
weenenewseswswneenenwwswwesesew
eneenenwnenenwnenenwewnwneneswnenwswnenw
swseswswseswswswswswswenwswswswseswnwswnwsw
eeeenwseeswneeesesw
nwneswswnenweneswnwwnwsewnweneenwwnw
eneneenenenwneswnwswnenwnwnenewnwnesene
swswsewswswswswswswneswsweewswwswswwne
wwwswwwwwwwwwswwswwnwneeww
seeseesesenwsenwseseseseseseseseswsesene
swnesweeeseswnwwnwneeenwnweee
nwwnewwwwsewwnwwnwneswnwwswesew
seseneeseeeseeseeseweeeswesene
seswwwseewnwswwwnwwswnewwswww
neeneneeneeenewneeseeeenesenw
wwnwwwwwnewnwwswwwwnww
sesenwseeseseseseseseseseseseseseseseswnw
neneeneeeneswneeneswnewenwwneee
nesenwneneneeseswsenenenenenenewnwenenwne
neenenenwnwsenesenwsenwnwswnenwne
swnwnwnweseseeeeeeeseseweswswseee
neneneneswnenenwneneneneneneeneneswnenene
newseneneenwnweweeeeseeeswew
eeeseeeeeeswseesenesesenwnwese
nwnewswnweneneneseenwswwnwswesesesene
neswesesesweenwseseeeeneseeeesee
eeesenweweeesenweseeeeswesesenw
seewesesenenwnewsewneeswnwsweesenw
nwwnesenwnenenwnenesenwnwnewnwnwnwnenw
sesenwseswseeseswnesweseseswnwwswseswsesw
nwneneneeswnwenwnwswnenwnenenw
eeseseseenewseswseswseeseneewee
swswsesesesewsenenwswweswswswsesenw
sewswswswswswswswneseswseswswsese
nesenenenwwnewneneneneseneseneeneswenene
nwswnwnwnwnwsenwneswwnewnesenenwneeneenw
swwwwwnwwewswnwwwwwenenwwsew
ewnwswswseenwsweeneneweeseenwwswne
nwseseswnwseseenwewseseseseseesesee
seeeseweseeeeeeneseeswseswsenw
wwewswneswswwwwwnesewwwswsww
nwnwwwnwwsewnwswnwewwnwnewwnww
swswswswseseswwswsesewswswswsweswnwnwene
nenwnwnwnwnwsenwnwnwnwnwnwwwnwnwnwsenwnw
nwswswnwnwsewsenwnwenwwnwnwnwnwnewnene
wswseeeswnwswseseneswesewswneswwsw
nwnwnwnwnenwnwswsenenwneeswsenenwnwswswnw
neewswneswnwseswnwnwnewnwwnwwwnw
nwnwseewneseneeeesewsesesee
nenesewnenenenenenenwne
seeseseseseswsesesesenwseseseswsesew
sewsenwwneneneeswseneeeeenenenwe
senwenwnwnwwenwnwnwnwnww
swnwsenwnwnwsenwswnenwnwswnwenenwnwsenene
seesenesesewsewnwnwswwsweenesewswsw
swenwnenewnenenwneswnwnenenewswnenee
wseesenwnwneswswsesesweswswseseswswswsw
eseseseseseseeseesesewseeseenwwee
eeneweseesweeeewneeewenenw
eeeneeswneeeneeeee
neneneeneswnwnenwnewnwneenewseenewew
eeswnwnenwnwneswnwnwwnwnwnwnwnw
swwswwwwwewswnwnwswsesw
seseseseswnwswseswne
sesesesenwnewenwnweseseseseeseseesew
wnesewwsewwwwwwnwsweswwnww
nwnenenwnewnenwnwseswneswnenenenenenwnwnwnw
nwseeswswswswswneseseswswswswswswsewswswsw
sesesewswneswseseseswseswsesesene
eseeseeseeeewsenwseeeeswsenwseese
eswswenenwnweenesenwsw
senwnwnwnwnwnenenwnenenwne
neswnwnwnwsenwnewnwnwneeewnenwswnenenenw
seeseeeseeneseswnewnewseeseeseesw
swswseswwswneswwswswswswweswswnenesw
eeenweseeseseeeeeseneeeswwe
nwnwnwnweeswnenwnenwnwnenwwnwswnwnewsene
neneswswwswneswswnewnwswswe
wwwwwwwweewsenwswwnwneswnwwsw
ewswseswnewsenwnwnwsenwnwsewseeseesw
sesesesenenwseswseswseseseseswwseswswe
neseweseseseseseseesewseseesesenese
nenenenenenesenewenenenwsewsenenene
weenesweseswesenwseeenwsese
weeseneseswswseswnwseseseesewnesesene
eeseneswswsweeeeseenwsenwsesenweew
swswnweswswswwseswswwwnwswwwwwswsw
seswswnwwnenwnenenwnwnwnwnwnwnwnw
swswseswswswswnwnwswswswsweswswseswswsw
ewswswswswswnwnenwswsesweswseswswswsww
nwwswwwwnewwwwnwnw
wnwewsenwwwwwwwwwewnewwww
swwsewwwwwwnenwnwwneewswwene
sweesweswwwneseswswnenwwseseewwse
swswwswneswweeswnwwenwswswseswswsw
nenwenenwswsenwnwswnenwenenwnwnwswswnenw
wwsewwwwnewwww
nweseeneeneswenesenwenwneneswnwne
esweswswswswseseswnwswswwswwnwwswnw
wwwsenwnewswwnwswwswwnwseseewwsw
seseswneswneseseswnenwswswswwseswswswse
swneeeneeneswnweneneneswswneneeneene
swswwswnwwnweeswswswwswwswswswswene
swswswwswswswswswnwneswswseswswswswswsw
seswwnwseswseswswswswsesesenewseseswsesene
enwnwseeneneeweneseeswnewnenewseene
nwswnesenwnewswswweenewewsesesesewne
enenwneseswneeneneenwneeenenesweee
nwnwseswnwnwwnwnwnwnenwnwnwnwnwnwnwnw
seseeseseseseeneweeeeweseseesee
enwnenwnwnewnwnwnenwenenwnwnwwnwnenw
eneseswnwnwnwnweswnwsewneswnwsenenenw
enenenwnwswnenenesenesewnenesene
eeneneeewseneneeeneswnee
seseswswseseswnwswenwswswswswseseeswsese
nwewswwwswnenesewwswwnwwwwewwsw
nwnwnwnweswnwnwnwnwnwnwnwnwnwnwnw
sewnenwnewnwneseneseswsenwnwnwswnenesesw
swswnwswwneeneewwewswswswnwwsew
newnenwnenwnwnwnwnenesenwnwwenwnwnwnw
wswwswwwwwswewwwwwnw
ewenweeesesesewseneseneeseswew
senewwswswsewnenwswewwwswwenenese
seswseeseseswesewseenweesenenesese
seeweseeneeeeeseenweeeewsee
neenweenewneesw
swnesenwsweeeswnwswwwseswneeswww
nwnwnwnenenweswsenwnwwneswnwnwnw
nesewswwwwewsww
seseseswseswswsesenwseswswswsesese
neenenenwneneeneenwswneneneneseneneene
nwnenwnwsewnwnenwnwnenenwnwnesenwnwnenwne
enenwneswwwnewswwswwswwwnwnesenese
seewenwseewswnesesesenwswswseswsewsw
swseswneswseseswswseswneswsenesesenwwsese
neneneewneneneneneneneseneswnwsenwneene
wnewewwwwwwsewsewwwwwneww
wswseeswsesesenwsesenwseseeswswswswse
eeenwswnwnesweeeeseeeeewesewe
swwwswwsesewwneenwswnenewsewwww
swswswswswseswswswswswneswsw
nwesewnwnwnwwnwnwnewnwnwwe
nwwwwneswwsesenesenwneenewwwwnw
seseswswswseneswswswswswnwswseseswsesewene
sweseseseseseeseseseseenwsesesesesesenw
swswnwswswswswswswwseswwnwswswswswswse
nenenenwnewsenenene
nwnwswwnwneeneswnwenesweswswwswe
nwnwnwnenwnenwnwneswneneswneneswnesenwswne
nwneseswswswswswseneswswwsesw
wnwswnwnwswswswswenenenwswswwsweswee
seswseseswseswseseeseswswneswswswswnwsese
seseneseseseseseesesesesesesesew
seseneswnwsesenwsesesewswseseswnw
nwswwnewwswwnewewwswesenwsww
swswswwseseseswwswswneswswswneswsweswsw
newnenenenwnenenwnwneeneswnenenwnenenw
swwswnweenewwsenwnenwenenweswsenwwne
seswnesesesenewseseseseseseswseswsewse
eseseeeeseeewwseesewsew
swswswswsweswswswswnwseswswwwswenenwsw
eswswswswswwswneseswswwswswswswwwsw
nenwswsenenwnenenenwnenenweswnenenw
swnwwewwwneeewwnwwswnwnwnenwsw
swwswseswswwswwwwswewnwsw
swwswewwswewnwnweewwnwneswwnw
senwnwseswswsesweswseeswwnwwesweswsw
wswnwwwwnwswwwwnewwswseswwsewsesw
swnwswswseswswswsesweseswseswsewsenwse
seswsenwswswnwswnwswswswenwewswswseene
eneweneswnwseeswneswwnwwsenwenww
eeeeenwswneseewseesenwewseneswse
nwnenwneneewnenenenwnenwnenwneseneseswnee
swneneewswswwswswewwswswnwnweswswe
sesesewseseseswseswsesesesesene
nwseeswseseweswswwe
nwnewnwewwwswwswwnwwwwswwnwe
wseneeeeneswweenwseeenwsewee
swsenwswseseswsesesenesesewwseneseswsese
seesesewsesesenewseseswseesenwsesesesese
swswsweseswnwswswswsesenw
neenwswswswswneneweswwneseeswneswsww
swwwsweswwnweswswwwnewwnwswwww
nwnwnwenenwnwneseswsenwsweswnwnwnewnw
wwwnewswwwswwwwwwwww
sweseseseseneneneseseseseseseseewwse
neeneeneeeweneneneseneneeenene
nwenwwwnwnwnwnwwnwnwnwsewnewnww
senwnwnwesweweewese
eseweeeeeseenweeeeeseewese
swneswsesesewswnwswseswneswseseee
neeseneeeneseeeenwswweeenenwnee
esweneenenwneeenesweeeeeeee
nwwwnwwwwwnwnwwwnwenewnwsewe
esenenweseeenweneneeneeneenenene
seeeeeeeeewenwseneeeeeee
swswnwwsenenwneneneneneseswenesenenew
senenwnewnwnwnenenwsenenwne
nenenwnewneneseneenenesenwnwneswnewne
sewswwneswwswwswewnwwswnwswwwsww
swswseswneswswwwswswnesewwwswswwwsw
swswneswwswwswswnwswwwswswswswseswe
nwenwwnwwnwnwnwswwnwwenwnwenwnww
swseswswswswwwnweswenw
senweswseseseswswswswseseswswwseswsese
neewwwnwnwwsenewsweswwnwswsesew
swseswseswneswseseseseseseseswnesesewswsw
wwwneenwwnwnwwneswwswnwsenwwnwswnw
neseseswwneseseseseswnwseswseswwseswsesw
nenenenenewwneeswweneeswweenwe
sweeswswswswswwwseseesenww
neneseneeswnewnwnewwsenwneneseenenene
nwnwsenwnenwnwnenwsenenwswnwnwnenwnwnwne
eeeeeeeswweeseneneeeeeneeenw
nenwnwsenwnenwnwwnwnwwnwnwnenwnwsenwnwnwnw
enwwseenwnwswsenwnwnwnwne
eeeeneneeeewseesweeeeeee
wnenwseweswnwnwnwwswewwnwnwnenwesw
eeneeweseeeneeseeneeeeweee
swnwnwenwwneseseesw
nwwwwswwwwwnwwnwnwwne
sewsweswenwnwneswwnenwnenewswesene
seeseeseswswneneeseswsesewswnwnwswsw
wseswnwnwnwseneenwsewsenwswew
wwneneswwswwswneswneseesenwwswsesw
wwwswnesenesewwwswswsewswswwwnw
nwnwswnwnwwwnwnwnwnwnwwnwnwnwewnwe
neneneneeswneneneneneneneneneene
ewseneseseseseeeseeseseseesesese
eeeeewnweseeesweewseeeeswnw
seswswswswwsweswswswswswswswsw
swswseseswwswsenwswswseswswseeseswswsw
swnwenwwwwwnwnewnenwswswenwsewnwnwnw
nwnwsenwnwswswwnwneenwnwseewnwsenwnese
enwnwnenwnwnwnenwnesenwswnenwnwnenenene
eswseswneenwenwnwseeewswseeneswnw
wwswnwswswswswseswswsweswswnewwsww
seeenwnenenenenesenenewneeenene
seswseseswswswswswseswswnwsw
nwswnwnesenenwneswnwnwnwnenwnwenwnwwnw
nweeseseswswswseswswseswseseseswnwseswsesw
eenweeeeeeneeeseeenwswnwese
swswswswswswswswsewnwswswswseswseswswswne
eswswnwnwnesweenwnwnww
neweseseneewsenweswsewseseeswwee
nwsenwenwnwnwwnwseswenwnwnwnwnenwwew
nwwnwnwwweseswnwww
swwewnwswswwwnenweewew
nwnwnwnwwnwswnenwsenenwwwnwsenwww
seeneseseeeeweweseenweseseswe
eesenwwewseesesenweeeseeenesw
swswswswswswswswswswswswnwswwswswswswee
wneneseneneesenenwneenenenwne
nenewneenenenenenenenenwnene
nesenwswnwnenwnwnwnenwswnwswnwnenene
nwwseneneneseneeenenenesweneneenwne
eseeenweesweeeeeneseeewenese
swwseseseswneswseseswswsweswsenwseswswse
seeeesenwswesewswnwseseseneenwwe
sweneeeweenenwneeeeeseseeseeew
neeswseweeeseseeeeneesenwsese
swwswwwswswswswswswswwneswwswswnese
senwneneswneeeswneneneseneneenwnewnenene
wwwwswwwwsewwwnww
ewswwwswswwswnwswwwwsewwwwnewsw
wnwswswnwneseseneesenenwsewnw
enwsweseeeeeeseneseseenweeeee
enewneeneeeeeneseeeeww
eweweneeseseeseseeeeeeeeee
swsenwweewwnwwnw
nenwseweneneseswnenwnewswneeew
eeswneneneseeneeseenewenwnweene
eeeeeeneneeesw"#;

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
