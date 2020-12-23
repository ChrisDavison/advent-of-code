use crate::prelude::*;

pub fn day15() -> Result<()> {
    let data = INPUT;
    let data = data
        .lines()
        .filter_map(|x| Some(x.parse().ok()?))
        .collect::<Vec<_>>();

    part1(&data)?;
    part2(&data)?;
    Ok(())
}

fn solve(data: &[usize], target: usize) -> Result<usize> {
    let mut last_num = 0;
    let mut history = vec![0usize; target];

    for (turn, number) in data.iter().enumerate() {
        history[*number] = turn + 1;
        last_num = *number;
    }

    for turn in data.len()..target {
        let last_spoken = &mut history[last_num];
        let last_turn = *last_spoken;
        *last_spoken = turn;
        last_num = if last_turn == 0 { 0 } else { turn - last_turn };
    }

    Ok(last_num)
}

fn part1(data: &[usize]) -> Result<usize> {
    let last_spoken = solve(data, 2020)?;
    println!("2020 15.1 -> {}", last_spoken);
    Ok(last_spoken)
}

fn part2(data: &[usize]) -> Result<usize> {
    let last_spoken = solve(data, 30_000_000)?;
    println!("2020 15.1 -> {}", last_spoken);
    Ok(last_spoken)
}

#[test]
fn d15_1_ex() {
    assert_eq!(part1(&[1, 3, 2]).unwrap(), 1);
    assert_eq!(part1(&[2, 1, 3]).unwrap(), 10);
    assert_eq!(part1(&[1, 2, 3]).unwrap(), 27);
    assert_eq!(part1(&[2, 3, 1]).unwrap(), 78);
    assert_eq!(part1(&[3, 2, 1]).unwrap(), 438);
    assert_eq!(part1(&[3, 1, 2]).unwrap(), 1836);
    assert_eq!(part1(&[0, 3, 6]).unwrap(), 436);
    assert_eq!(part2(&[0, 3, 6]).unwrap(), 175594);
}

const INPUT: &str = "1
0
16
5
17
4";
