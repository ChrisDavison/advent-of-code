#![allow(dead_code, unused_variables, unused_mut, unreachable_code)]
use anyhow::{anyhow, Result};

pub fn day22() -> Result<()> {
    let (a, b) = parse(&INPUT);
    println!("{:?}", a);
    println!("AoC2020 22.1 -> {}", part1(&a, &b)?);
    println!("AoC2020 22.2 -> {}", part2(&INPUT)?);
    Ok(())
}

fn part1(pl_a: &[usize], pl_b: &[usize]) -> Result<String> {
    let mut deck_a = pl_a.to_vec();
    let mut deck_b = pl_b.to_vec();

    while !(deck_a.is_empty() || deck_b.is_empty()) {
        let top_a = deck_a.remove(0);
        let top_b = deck_b.remove(0);
        if top_a > top_b {
            deck_a.push(top_a);
            deck_a.push(top_b);
        } else {
            deck_b.push(top_b);
            deck_b.push(top_a);
        }
    }

    let winner = if deck_a.is_empty() { deck_b } else { deck_a };
    let muls: Vec<usize> = (1..=winner.len()).rev().collect();
    let score = winner.iter().zip(muls).map(|(a, b)| a * b).sum::<usize>();
    Ok(format!("{}", score))
}

fn part2(data: &str) -> Result<String> {
    Err(anyhow!("Part 2 not implemented"))
    //Ok(format!("{}", 0))
}

fn parse(data: &str) -> (Vec<usize>, Vec<usize>) {
    let mut parts = data.split("\n\n");
    let a = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .filter_map(|x| Some(x.parse().ok()?))
        .collect();
    let b = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .filter_map(|x| Some(x.parse().ok()?))
        .collect();
    (a, b)
}

const INPUT: &str = r#"Player 1:
1
10
28
29
13
11
35
7
43
8
30
25
4
5
17
32
22
39
50
46
16
26
45
38
21

Player 2:
19
40
2
12
49
23
34
47
9
14
20
24
42
37
48
44
27
6
33
18
15
3
36
41
31"#;

const SAMPLE: &str = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#;
