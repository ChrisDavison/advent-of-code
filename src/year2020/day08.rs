use anyhow::Result;
use std::num::ParseIntError;

const DAY: usize = 8;

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

impl std::str::FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();
        let num = parts[1].parse::<i64>()?;
        match parts[0] {
            "acc" => Ok(Instruction::Acc(num)),
            "jmp" => Ok(Instruction::Jmp(num)),
            "nop" => Ok(Instruction::Nop(num)),
            _ => Ok(Instruction::Nop(num)),
        }
    }
}

pub fn solve() -> Result<()> {
    let data = std::fs::read_to_string(format!("input/day{}.txt", DAY))?;
    let instructions: Vec<Instruction> = data
        .split('\n')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .filter_map(|x| Some(x.parse().ok()?))
        .collect();

    let (total, _) = part1(&instructions);
    println!("AoC2020 {}.1 - {}", DAY, total);

    let total2 = part2(&instructions);
    println!("AoC2020 {}.2 - {}", DAY, total2);
    Ok(())
}

fn part1(instructions: &[Instruction]) -> (i64, bool) {
    let mut acc = 0;
    let mut cur_idx = 0;
    let mut visited: Vec<bool> = (0..instructions.len()).map(|_| false).collect();
    loop {
        if cur_idx > instructions.len() - 1 {
            return (acc, true);
        }
        if visited[cur_idx] {
            return (acc, false);
        }
        visited[cur_idx] = true;
        match instructions[cur_idx] {
            Instruction::Acc(n) => {
                acc += n;
                cur_idx += 1;
            }
            Instruction::Jmp(n) => cur_idx = ((cur_idx as i64) + n) as usize,
            Instruction::Nop(_) => cur_idx += 1,
        }
    }
}

fn part2(instructions: &[Instruction]) -> i64 {
    let mut idx = 0;
    loop {
        let mut local_instructions: Vec<Instruction> = instructions.to_vec();
        println!("{} {:?}", idx, instructions[idx]);
        match instructions[idx] {
            Instruction::Nop(n) => local_instructions[idx] = Instruction::Jmp(n),
            Instruction::Jmp(n) => local_instructions[idx] = Instruction::Nop(n),
            _ => {
                idx += 1;
                continue;
            }
        };
        let (acc, ran_to_end) = part1(&local_instructions);
        if ran_to_end {
            return acc;
        }
        idx += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let data = "nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6";
        let instructions: Vec<Instruction> = data
            .trim()
            .split('\n')
            .map(|x| x.trim())
            .filter_map(|x| Some(x.parse().ok()?))
            .collect();
        assert_eq!(part1(&instructions), (5, false));
    }

    #[test]
    fn example_part2() {
        let data = "nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6";
        let instructions: Vec<Instruction> = data
            .trim()
            .split('\n')
            .map(|x| x.trim())
            .filter_map(|x| Some(x.parse().ok()?))
            .collect();
        assert_eq!(part2(&instructions), 8);
    }
}
