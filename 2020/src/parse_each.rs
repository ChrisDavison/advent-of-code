use std::str::FromStr;

#[allow(dead_code)]
pub fn parse_each<T: FromStr, U: Iterator<Item = impl ToString>>(data: U) -> Vec<T> {
    data.filter_map(|x| x.to_string().parse().ok())
        .collect()
}
