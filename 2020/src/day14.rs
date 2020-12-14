use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Bit {
    One,
    Zero,
    X,
}

type BitMask = [Bit; 36];

#[derive(Debug, Copy, Clone)]
enum Op {
    Mem(usize, usize),
    Mask(BitMask),
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Op::Mem(addr, val) => write!(f, "mem[{}] = {}", addr, val),
            Op::Mask(mask) => {
                let mask_str = mask
                    .iter()
                    .map(|c| match c {
                        Bit::One => '1',
                        Bit::Zero => '0',
                        Bit::X => 'x',
                    })
                    .collect::<String>();
                write!(f, "m {}", mask_str)
            }
        }
    }
}

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(instruction: &str) -> anyhow::Result<Self> {
        let mut ins_parts = instruction.split(" = ");
        let action = ins_parts.next().ok_or(anyhow!("Action missing for op"))?;
        let value = ins_parts.next().ok_or(anyhow!("Value missing for op"))?;
        if action == "mask" {
            let mut bitmask = [Bit::X; 36];
            for (i, c) in value.chars().enumerate() {
                match c {
                    '1' => bitmask[i] = Bit::One,
                    '0' => bitmask[i] = Bit::Zero,
                    _ => bitmask[i] = Bit::X,
                }
            }
            Ok(Op::Mask(bitmask))
        } else {
            Ok(Op::Mem(
                action[4..action.len() - 1].parse()?,
                value.parse()?,
            ))
        }
    }
}

pub fn day14(data: &str) -> Result<()> {
    let instructions: Vec<Op> = data.lines().filter_map(|l| Some(l.parse().ok()?)).collect();
    part_1(&instructions)?;
    part_2(&instructions)?;
    Ok(())
}

fn apply_mask(value: usize, mask: BitMask) -> String {
    format!("{:036b}", value)
        .chars()
        .enumerate()
        .map(|(i, x)| match mask[i] {
            Bit::One => '1',
            Bit::Zero => '0',
            Bit::X => x,
        })
        .collect::<String>()
}

fn apply_mask_v2(value: usize, mask: BitMask) -> String {
    format!("{:036b}", value)
        .chars()
        .enumerate()
        .map(|(i, x)| match mask[i] {
            Bit::One => '1',
            Bit::Zero => x,
            Bit::X => 'x',
        })
        .collect::<String>()
}

fn update_memory(
    mem: &mut HashMap<usize, usize>,
    addr: usize,
    value: usize,
    mask: BitMask,
) -> Result<()> {
    let valstr_masked = apply_mask(value, mask);
    let val = usize::from_str_radix(&valstr_masked, 2).map_err(|e| anyhow!("Apply mask: {}", e))?;

    mem.insert(addr, val);
    Ok(())
}

fn bit_permutations(n: usize) -> Vec<Vec<char>> {
    let n_permutations: usize = 2usize.pow(n as u32);
    (0..n_permutations)
        .map(|i| format!("{:0n$b}", i, n = n).chars().collect::<Vec<char>>())
        .collect()
}

fn update_memory_v2(
    mem: &mut HashMap<usize, usize>,
    addr: usize,
    value: usize,
    mask: BitMask,
) -> Result<()> {
    let x_locs: Vec<usize> = mask
        .iter()
        .enumerate()
        .filter(|(_i, &c)| c == Bit::X)
        .map(|(i, _)| i)
        .collect();

    let masked_start_addr: Vec<char> = apply_mask_v2(addr, mask).chars().collect();
    for perm in bit_permutations(x_locs.len()) {
        // e.g. vec[0, 0, 0]
        let mut addr_perm = masked_start_addr.clone();
        x_locs
            .iter()
            .zip(perm.iter())
            .for_each(|(&i, &bit)| addr_perm[i] = bit);
        let actual_addr = addr_perm.iter().collect::<String>();
        let addr_new = usize::from_str_radix(&actual_addr, 2)
            .map_err(|e| anyhow!("Failed to convert masked addr from binary: {}", e))?;
        mem.insert(addr_new, value);
    }
    Ok(())
}

fn part_1(instructions: &[Op]) -> Result<usize> {
    let mut current_mask = [Bit::X; 36];
    let mut memory: HashMap<usize, usize> = HashMap::new();
    for &op in instructions {
        match op {
            Op::Mask(bitmask) => current_mask = bitmask,
            Op::Mem(addr, value) => update_memory(&mut memory, addr, value, current_mask)?,
        }
    }
    let result = memory.iter().map(|(_k, v)| v).sum();
    println!("AoC2020 14.1 -> {}", result);
    Ok(result)
}

fn part_2(instructions: &[Op]) -> Result<usize> {
    let mut current_mask = [Bit::X; 36];
    let mut memory: HashMap<usize, usize> = HashMap::new();
    for &op in instructions {
        match op {
            Op::Mask(bitmask) => current_mask = bitmask,
            Op::Mem(addr, value) => update_memory_v2(&mut memory, addr, value, current_mask)?,
        }
    }
    let result = memory.iter().map(|(_k, v)| v).sum();
    println!("AoC2020 14.2 -> {}", result);
    Ok(result)
}
