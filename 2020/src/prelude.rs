pub use crate::bool_xor;
pub use crate::strides::Strides;
pub use anyhow::{anyhow, Result};
pub use itertools::Itertools;
pub use regex::Regex;
pub use std::collections::{HashMap, HashSet, VecDeque};
pub use std::fmt::{self, Display};
pub use std::iter::repeat;
pub use std::str::FromStr;

#[allow(dead_code)]
pub fn parse_each<T: FromStr, U: Iterator<Item = impl ToString>>(data: U) -> Vec<T> {
    data.filter_map(|x| Some(x.to_string().parse().ok()?))
        .collect()
}
