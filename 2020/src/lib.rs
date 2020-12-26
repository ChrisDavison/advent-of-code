#![feature(core_intrinsics)]
mod bool_xor;
mod parse_each;
mod quick_dict;
mod strides;

pub use crate::parse_each::parse_each;
pub use crate::strides::Strides;
pub use anyhow::{anyhow, Result};
pub use itertools::Itertools;
pub use regex::Regex;
pub use std::collections::{HashMap, HashSet, VecDeque};
pub use std::fmt::{self, Display};
pub use std::hash::{Hash, Hasher};
pub use std::iter::repeat;
pub use std::str::FromStr;
pub use std::time::Instant;

#[allow(dead_code)]
pub fn as_ms(delta: std::time::Duration) -> u128 {
    delta.as_millis()
}

#[macro_export]
macro_rules! time_solution {
    ($day:ident, $body:expr) => {
        let now = std::time::Instant::now();
        let res = $body;
        match res {
            Ok(_) => println!("    Time: {:.2}ms", now.elapsed().as_nanos() / 1_000_000),
            Err(e) => eprintln!("D{}: {}", $day + 1, e),
        }
    };
}
