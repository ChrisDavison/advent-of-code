use aoc2020::*;

type Deck = VecDeque<usize>;

pub fn day22() -> Result<String> {
    let input = include_str!("../input/day22");
    let (a, b) = parse(input);
    Ok(format!(
        "2020 22.1 -> {}\n2020 22.2 -> {}",
        part1(&a, &b)?,
        part2(&a, &b)?
    ))
}

fn score(deck: &Deck) -> usize {
    deck.iter()
        .zip((1..=deck.len()).rev())
        .map(|(a, b)| a * b)
        .sum::<usize>()
}

fn part1(a: &Deck, b: &Deck) -> Result<String> {
    let mut a = a.clone();
    let mut b = b.clone();
    while !(a.is_empty() || b.is_empty()) {
        let top_a = a.pop_front().unwrap();
        let top_b = b.pop_front().unwrap();
        if top_a > top_b {
            a.extend(&[top_a, top_b]);
        } else {
            b.extend(&[top_b, top_a]);
        }
    }

    let winner = if a.is_empty() { b } else { a };
    Ok(format!("{}", score(&winner)))
}

fn part2(a: &Deck, b: &Deck) -> Result<String> {
    let (winners_deck, _a_won) = play_recursive_game(a, b);
    Ok(format!("{}", score(&winners_deck)))
}

fn play_recursive_game(a: &Deck, b: &Deck) -> (Deck, bool) {
    let mut a = a.clone();
    let mut b = b.clone();

    let mut history: HashSet<String> = HashSet::new();

    while !a.is_empty() && !b.is_empty() {
        if !history.insert(stringify_decks(&a, &b)) {
            return (a, true);
        }
        let top_a = a.pop_front().unwrap();
        let top_b = b.pop_front().unwrap();
        let a_won = if top_a <= a.len() && top_b <= b.len() {
            let (_, a_won) = play_recursive_game(
                &a.clone().iter().take(top_a).copied().collect(),
                &b.clone().iter().take(top_b).copied().collect(),
            );
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
        (a, true)
    } else {
        (b, false)
    }
}

fn stringify_decks(a: &Deck, b: &Deck) -> String {
    format!("{:?}|{:?}", &a, &b)
}

fn parse(data: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut parts = data.split("\n\n");
    let a = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .filter_map(|x| x.parse().ok())
        .collect();
    let b = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .filter_map(|x| x.parse().ok())
        .collect();
    (a, b)
}

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
