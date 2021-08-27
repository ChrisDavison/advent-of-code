use aoc2020::*;

pub fn day07() -> Result<String> {
    let data = include_str!("../input/day07");
    let tidy_data = data.lines().map(|x| x.trim()).filter(|x| !x.is_empty());
    let bagmap = create_bagmap(tidy_data);

    let n_containing_gold = bags_containing_gold(&bagmap) - 1;
    let n_inside_gold = total_bags(&bagmap, "shiny gold") - 1;
    let output = format!(
        "2020 07.1 -> {}\n2020 07.2 -> {}",
        n_containing_gold, n_inside_gold
    );
    Ok(output)
}

type BagMap = HashMap<String, Vec<(String, usize)>>;

type BagCache = HashMap<String, bool>;

fn create_bagmap<'a>(data: impl Iterator<Item = &'a str>) -> BagMap {
    let mut bagmap: BagMap = HashMap::new();
    for line in data {
        let mut parts = line.split("contain").take(2).map(|x| x.trim());
        let source = parts
            .next()
            .map(|x| x.trim().trim_end_matches("bags").trim());
        let inner_bags = parts
            .next()
            .map(|x| x.split(',').filter_map(|x| parse_bag(x).ok()).collect());
        bagmap.insert(source.unwrap().to_string(), inner_bags.unwrap());
    }
    bagmap
}

fn parse_bag(s: &str) -> Result<(String, usize)> {
    let mut parts = s
        .trim_end_matches('.')
        .trim()
        .split(' ')
        .filter(|x| !["bag", "bags"].contains(x));
    let num = parts.next().unwrap().parse()?;
    Ok((parts.collect::<Vec<&str>>().join(" "), num))
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
        .filter(|key| contains_gold(bagmap, key, &mut cache))
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
