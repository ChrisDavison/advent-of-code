use aoc2020::*;

fn main() -> Result<()> {
    let data = include_str!("input");
    let mut data = parse_each(data.lines());
    data.sort_unstable();
    part_1(&data);
    part_2(&data);
    Ok(())
}

fn part_1(data: &[usize]) -> usize {
    // start with (1,1) in acc, as the wall-socket is a 1, and our device is a 3
    let (ones, threes) =
        data.windows(2)
            .fold((1, 1), |(ones, threes), val| match val[1] - val[0] {
                1 => (ones + 1, threes),
                3 => (ones, threes + 1),
                _ => unreachable!(),
            });
    println!("2020 10.1 -> {}", ones * threes);
    ones * threes
}

fn part_2(data: &[usize]) -> usize {
    let result = data.windows(2).collect::<Vec<_>>();
    let split = result.split(|n| n[1] - n[0] == 3).collect::<Vec<_>>();
    let product = 2 * split
        .iter()
        .map(|n| match n.len() {
            4 => 7,
            3 => 4,
            2 => 2,
            _ => 1,
        })
        .product::<usize>();
    println!("2020 10.2 -> {}", product);
    product
}

#[allow(dead_code)]
const SAMPLE: &str = r#"16
10
15
5
1
11
7
19
6
12
4"#;

#[allow(dead_code)]
const SAMPLE2: &str = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;
