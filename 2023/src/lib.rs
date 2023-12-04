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
    ($year:literal, $daynum:literal, $day:ident) => {
        let now = std::time::Instant::now();
        match $day::$day() {
            Ok((a, b)) => println!(
                "{}.1 => {}\n{}.2 => {}\n\tTime {}ms\n--------------------",
                $daynum,
                a,
                $daynum,
                b,
                now.elapsed().as_millis(),
            ),
            Err(e) => eprintln!("D{}: {}", $daynum + 1, e),
        }
    };
}
