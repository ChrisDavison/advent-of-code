use aoc2020::*;

fn main() -> Result<()> {
    let data = include_str!("input");
    let instructions: Vec<Instruction> = parse_each(data.lines());

    let (total, _) = part1(&instructions);
    println!("2020 08.1 -> {}", total);

    let total2 = part2(&instructions);
    println!("2020 08.2 -> {}", total2);
    Ok(())
}

fn part1(instructions: &[Instruction]) -> (i64, bool) {
    let mut acc = 0;
    let mut cur_idx: i64 = 0;
    let mut visited: Vec<bool> = (0..instructions.len()).map(|_| false).collect();
    loop {
        if cur_idx as usize > instructions.len() - 1 {
            return (acc, true);
        }
        if visited[cur_idx as usize] {
            return (acc, false);
        }
        visited[cur_idx as usize] = true;
        let (acc_mod, idx_mod) = match instructions[cur_idx as usize] {
            Instruction::Acc(n) => (n, 1),
            Instruction::Jmp(n) => (0, n),
            Instruction::Nop(_) => (0, 1),
        };
        acc += acc_mod;
        cur_idx += idx_mod;
    }
}

fn part2(instructions: &[Instruction]) -> i64 {
    let mut idx = 0;
    loop {
        let mut local_instructions: Vec<Instruction> = instructions.to_vec();
        local_instructions[idx] = match instructions[idx] {
            Instruction::Nop(n) => Instruction::Jmp(n),
            Instruction::Jmp(n) => Instruction::Nop(n),
            Instruction::Acc(n) => Instruction::Acc(n),
        };
        let (acc, ran_to_end) = part1(&local_instructions);
        if ran_to_end {
            return acc;
        }
        idx += 1;
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();
        let num = parts[1].parse::<i64>()?;
        match parts[0] {
            "acc" => Ok(Instruction::Acc(num)),
            "jmp" => Ok(Instruction::Jmp(num)),
            "nop" => Ok(Instruction::Nop(num)),
            x => Err(anyhow!("Unrecognised operation `{}`", x)),
        }
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
            .lines()
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
            .lines()
            .map(|x| x.trim())
            .filter_map(|x| Some(x.parse().ok()?))
            .collect();
        assert_eq!(part2(&instructions), 8);
    }
}
