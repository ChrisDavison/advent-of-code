use anyhow::Result;
use std::collections::HashSet;

type Deck = Vec<usize>;

pub fn day22() -> Result<()> {
    let (a, b) = parse(&INPUT);
    println!("2020 22.1 -> {}", part1(&mut a.clone(), &mut b.clone())?);
    println!("2020 22.2 -> {}", part2(&mut a.clone(), &mut b.clone())?);
    Ok(())
}

fn score(deck: &Deck) -> usize {
    deck.iter()
        .zip((1..=deck.len()).rev())
        .map(|(a, b)| a * b)
        .sum::<usize>()
}

fn part1(a: &mut Deck, b: &mut Deck) -> Result<String> {
    while !(a.is_empty() || b.is_empty()) {
        let top_a = a.remove(0);
        let top_b = b.remove(0);
        if top_a > top_b {
            a.extend(&[top_a, top_b]);
        } else {
            b.extend(&[top_b, top_a]);
        }
    }

    let winner = if a.is_empty() { b } else { a };
    Ok(format!("{}", score(&winner)))
}

fn part2(mut a: &mut Deck, mut b: &mut Deck) -> Result<String> {
    let (winners_deck, _a_won) = play_recursive_game(&mut a, &mut b);
    Ok(format!("{}", score(&winners_deck)))
}

fn play_recursive_game(a: &mut Deck, b: &mut Deck) -> (Deck, bool) {
    let mut history: HashSet<String> = HashSet::new();

    while !a.is_empty() && !b.is_empty() {
        if history.contains(&stringify_decks(&a, &b)) {
            return (a.to_vec(), true);
        }
        history.insert(stringify_decks(&a, &b));
        let top_a = a.remove(0);
        let top_b = b.remove(0);
        let a_won = if top_a <= a.len() && top_b <= b.len() {
            let mut new_a = a.iter().take(top_a).copied().collect();
            let mut new_b = b.iter().take(top_b).copied().collect();
            let (_, a_won) = play_recursive_game(&mut new_a, &mut new_b);
            a_won
        } else {
            top_a > top_b
        };
        if a_won {
            a.extend(&[top_a, top_b]);
        } else {
            b.extend(&[top_b, top_a]);
        }
    }
    if b.is_empty() {
        (a.to_vec(), true)
    } else {
        (b.to_vec(), false)
    }
}

fn stringify_decks(a: &Deck, b: &Deck) -> String {
    format!("{:?}|{:?}", &a, &b)
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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
