#![feature(iter_array_chunks)]
mod bool_xor;
mod parse_each;
mod point2d;
mod quick_dict;
mod strides;

pub use crate::parse_each::parse_each;
pub use crate::point2d::Point2D;
pub use crate::strides::Strides;
pub use anyhow::{anyhow, Result};
pub use itertools::Itertools;
pub use lazy_static::lazy_static;
pub use regex::Regex;
pub use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
pub use std::fmt::{self, Display};
pub use std::hash::{Hash, Hasher};
pub use std::iter::repeat;
pub use std::str::FromStr;
pub use std::time::Instant;

#[macro_export]
macro_rules! time_solution {
    ($daynum:literal, $day:ident) => {
        let now = std::time::Instant::now();
        match $day::$day() {
            Ok((a, b)) => println!(
                "{}.1 => {}\n{}.2 => {}\n\tTime {}ms\n--------------------",
                $daynum,
                a.to_string(),
                $daynum,
                b.to_string(),
                now.elapsed().as_millis(),
            ),
            Err(e) => eprintln!("D{}: {}", $daynum + 1, e),
        }
    };
}

#[macro_export]
macro_rules! timed {
    ($partnum:literal, $part:ident, $data:ident) => {
        let now = std::time::Instant::now();
        match $part(&$data) {
            Ok(a) => {
                println!(
                    "Part {} ({}ms) => {}",
                    $partnum,
                    now.elapsed().as_millis(),
                    a.to_string(),
                )
            }
            Err(e) => eprintln!("Part {}: {}", $partnum, e),
        }
    };
}

pub fn numbers(s: &str) -> Vec<usize> {
    s.split(|c: char| !c.is_ascii_digit())
        .filter_map(|x| x.trim().parse().ok())
        .collect()
}

pub fn number_pairs(s: &str) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for ab in s
        .split(|c: char| !c.is_ascii_digit())
        .filter_map(|x| x.trim().parse().ok())
        .array_chunks::<2>()
    {
        let a = ab[0];
        let b = ab[1];
        out.push((a, b));
    }
    out
}

pub fn pairs<T: Clone + Copy>(v: Vec<T>) -> impl Iterator<Item = (T, T)> {
    let range = (1..=v.len()).collect::<Vec<usize>>();
    let mut i = 0;
    std::iter::from_fn(move || {
        for _ in &range {
            i += 2;
            if i > range.len() {
                break;
            }
            return Some((v[i - 2], v[i - 1]));
        }
        None
    })
}

pub fn noop<T: Display>(thing: T) -> Result<String> {
    Ok(format!("{}", thing))
}
