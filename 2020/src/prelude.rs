pub use crate::bool_xor;
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
pub fn parse_each<T: FromStr, U: Iterator<Item = impl ToString>>(data: U) -> Vec<T> {
    data.filter_map(|x| Some(x.to_string().parse().ok()?))
        .collect()
}

#[allow(dead_code)]
pub fn as_ms(delta: std::time::Duration) -> u128 {
    delta.as_nanos() / 1_000_000
}
