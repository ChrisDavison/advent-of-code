#![allow(dead_code, unused_variables, unused_mut, unreachable_code)]
use crate::prelude::*;

const NEIGHBOUR_DIRS: [(f32, f32); 6] = [
    (0.0, 1.0),   // east
    (0.0, -1.0),  // west
    (-0.5, 0.5),  // se
    (-0.5, -0.5), // sw
    (0.5, 0.5),   // ne
    (0.5, -0.5),  // nw
];

pub fn day24() -> Result<()> {
    let tiles: Vec<Hex> = parse_each(INPUT.lines());
    let mut renovated = get_starting_floor_state(&tiles);
    println!("AoC2020 24.1 -> {}", part1(&renovated)?);
    println!("AoC2020 24.2 -> {}", part2(&mut renovated)?);
    Ok(())
}

fn count_black(floor: &HashMap<Hex, bool>) -> usize {
    floor.iter().filter(|(k, v)| **v).count()
}

fn part1(state: &HashMap<Hex, bool>) -> Result<String> {
    Ok(format!("{}", count_black(&state)))
}

fn part2(mut state: &mut HashMap<Hex, bool>) -> Result<String> {
    let mut hexes_to_flip: Vec<Hex> = vec![];

    fill_grid(&mut state, 50);

    for i in 0..100 {
        // Start assuming nothing needs flipped
        hexes_to_flip.clear();

        // Go through every tile seen so far, and make sure we're
        // basically 1 wider, to ensure that the outer edge can flip
        // if necessary.
        add_neighbours(&mut state);

        // For every hex so far, check if it should should flip
        // if so, add it to the lisp
        for (tile, _) in &state.clone() {
            if should_flip(&tile, &mut state) {
                hexes_to_flip.push(*tile);
            }
        }

        // ========================================
        // Flip hexes that match the rules
        for hex in &hexes_to_flip {
            if let Some(e) = state.get_mut(hex) {
                *e = !*e;
            }
        }

        // =====================================================
        // Status message. How many black facing up on each day
        println!("Day {}: {}", i + 1, count_black(&state));
    }
    // Err(anyhow!("Part 2 not implemented"))
    Ok(format!("{}", count_black(&state)))
}

fn fill_grid(mut state: &mut HashMap<Hex, bool>, n: usize) {
    let mut offset = 0f32;
    for i in 0..2 * n {
        for j in 0..2 * n {
            let north = (i as f32 - n as f32) + offset;
            let east = (j as f32 - n as f32) + offset;
            let h = Hex { north, east };
            let _ = state.entry(h).or_insert(false);
        }
        offset = if offset == 0f32 { 0.5 } else { 0f32 };
    }
}

fn should_flip(tile: &Hex, history: &mut HashMap<Hex, bool>) -> bool {
    let n_black = NEIGHBOUR_DIRS
        .iter()
        .map(|(n, e)| Hex {
            north: tile.north + n,
            east: tile.east + e,
        })
        .filter(|n| history.entry(*n).or_insert(false).clone())
        .count();

    let tile_is_black = history.get(tile).unwrap();
    if *tile_is_black {
        n_black == 0 || n_black > 2
    } else {
        n_black == 2
    }
}

fn add_neighbours(mut state: &mut HashMap<Hex, bool>) {
    let mut to_add = HashSet::new();
    for tile in state.keys() {
        for dir in &NEIGHBOUR_DIRS {
            let neighbour = Hex {
                north: tile.north + dir.0,
                east: tile.east + dir.1,
            };
            if !state.contains_key(&neighbour) {
                to_add.insert(neighbour);
            }
        }
    }
    for neighbour in to_add {
        state.insert(neighbour, false);
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
    north: f32,
    east: f32,
}

impl Hex {
    fn print_neighbours(&self) {
        let nw = Hex {
            north: self.north + 0.5,
            east: self.east - 0.5,
        };
        let ne = Hex {
            north: self.north + 0.5,
            east: self.east + 0.5,
        };
        let sw = Hex {
            north: self.north - 0.5,
            east: self.east - 0.5,
        };
        let se = Hex {
            north: self.north - 0.5,
            east: self.east + 0.5,
        };
        let w = Hex {
            north: self.north,
            east: self.east - 1.0,
        };
        let e = Hex {
            north: self.north,
            east: self.east + 1.0,
        };

        let row0 = format!("\t{}\t{}", nw, ne);
        let row1 = format!("{}\t{}\t{}", w, self, e);
        let row2 = format!("\t{}\t{}", sw, se);
        println!("{}\n{}\n{}\n", row0, row1, row2);
    }
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
        let mut north = 0 as f32;
        let mut east = 0 as f32;
        let mut cs = s.chars();
        loop {
            let current = cs.next();
            match current {
                Some('s') => {
                    north -= 0.5;
                    if let Some('e') = cs.next() {
                        east += 0.5;
                    } else {
                        east -= 0.5;
                    }
                }
                Some('n') => {
                    north += 0.5;
                    if let Some('e') = cs.next() {
                        east += 0.5;
                    } else {
                        east -= 0.5;
                    }
                }
                Some('e') => {
                    east += 1.0;
                }
                Some('w') => {
                    east -= 1.0;
                }
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
