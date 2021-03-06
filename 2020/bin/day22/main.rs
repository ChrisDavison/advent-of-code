use aoc2020::*;

type Deck = VecDeque<usize>;

fn main() -> Result<()> {
    let input = include_str!("input");
    let (mut a, mut b) = parse(&input);
    println!("2020 22.1 -> {}", part1(&mut a.clone(), &mut b.clone())?);
    println!("2020 22.2 -> {}", part2(&mut a, &mut b)?);
    Ok(())
}

fn score(deck: &Deck) -> usize {
    deck.iter()
        .zip((1..=deck.len()).rev())
        .map(|(a, b)| a * b)
        .sum::<usize>()
}

fn part1<'a>(a: &'a mut Deck, b: &'a mut Deck) -> Result<String> {
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

fn part2(mut a: &mut Deck, mut b: &mut Deck) -> Result<String> {
    let (winners_deck, _a_won) = play_recursive_game(&mut a, &mut b);
    Ok(format!("{}", score(&winners_deck)))
}

fn play_recursive_game<'a>(a: &'a mut Deck, b: &'a mut Deck) -> (&'a Deck, bool) {
    let mut history: HashSet<String> = HashSet::new();

    while !a.is_empty() && !b.is_empty() {
        if !history.insert(stringify_decks(&a, &b)) {
            return (a, true);
        }
        let top_a = a.pop_front().unwrap();
        let top_b = b.pop_front().unwrap();
        let a_won = if top_a <= a.len() && top_b <= b.len() {
            let (_, a_won) = play_recursive_game(
                &mut a.clone().iter().take(top_a).copied().collect(),
                &mut b.clone().iter().take(top_b).copied().collect(),
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
