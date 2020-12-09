use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashMap;

const DAY: usize = 7;

pub fn day07() -> Result<()> {
    let data = std::fs::read_to_string(format!("input/day{}.in", DAY))?;
    let tidy_data: Vec<&str> = data
        .as_parallel_string()
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect();

    let bagmap = create_bagmap(&tidy_data);

    let n_containing_gold = bags_containing_gold(&bagmap) - 1;
    let n_inside_gold = total_bags(&bagmap, "shiny gold") - 1;

    println!("2020 {}-1 -> {}", DAY, n_containing_gold);
    println!("2020 {}-2 -> {}", DAY, n_inside_gold);
    Ok(())
}

type BagMap = HashMap<String, Vec<(String, usize)>>;

type BagCache = HashMap<String, bool>;

fn create_bagmap(data: &[&str]) -> BagMap {
    let mut bagmap: BagMap = HashMap::new();
    for line in data {
        let parts: Vec<_> = line.split("contain").take(2).map(|x| x.trim()).collect();
        let source = parts[0].trim().trim_end_matches("bags").trim();
        let inner_bags: Vec<_> = parts[1]
            .split(',')
            .map(parse_bag)
            .filter_map(|x| x.ok())
            .collect();
        bagmap.insert(source.to_string(), inner_bags);
    }
    bagmap
}

fn parse_bag(s: &str) -> Result<(String, usize)> {
    let parts: Vec<_> = s
        .trim_end_matches('.')
        .trim()
        .split(' ')
        .filter(|x| !["bag", "bags"].contains(x))
        .collect();
    let num = parts[0].parse()?;
    Ok((parts[1..].join(" "), num))
}

fn contains_gold(bagmap: &BagMap, bag: &str, cache: &mut BagCache) -> bool {
    if !cache.contains_key(bag) {
        let b = bagmap[bag]
            .iter()
            .any(|(b, _)| contains_gold(bagmap, b, cache));
        cache.insert(bag.to_string(), b);
    }
    cache[bag]
}

fn bags_containing_gold(bagmap: &BagMap) -> usize {
    let mut cache = HashMap::new();
    cache.insert(String::from("shiny gold"), true);
    bagmap
        .keys()
        .filter(|key| contains_gold(&bagmap, key, &mut cache))
        .count()
}

fn total_bags(bagmap: &BagMap, bag: &str) -> usize {
    // 1, as you're counting THIS bag
    // and then the sum of all the bags under this, multipled by how many
    // bags you must take
    1 + bagmap[bag]
        .iter()
        .map(|(bag, count)| count * total_bags(bagmap, bag))
        .sum::<usize>()
}
