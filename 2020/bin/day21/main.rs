use aoc2020::*;

fn main() -> Result<()> {
    let input = include_str!("input");
    let (ingredients, allergen_map) = parse_data(&input);

    println!("2020 21.1 -> {}", part1(&ingredients, &allergen_map)?);
    println!("2020 21.2 -> {}", part2(&allergen_map)?);
    Ok(())
}

fn part1(ingredients: &[HashSet<&str>], allergens: &HashMap<&str, &str>) -> Result<String> {
    let allergens = allergens.values().collect::<HashSet<_>>();
    let mut count = 0;
    for list in ingredients {
        for food in list {
            if !allergens.contains(&food) {
                count += 1;
            }
        }
    }
    Ok(format!("{}", count))
}

fn part2(allergens: &HashMap<&str, &str>) -> Result<String> {
    let mut allergens = allergens
        .iter()
        .map(|(k, v)| (*k, *v))
        .collect::<Vec<(&str, &str)>>();
    allergens.sort();
    let ingredients = allergens
        .iter()
        .map(|(_, v)| *v)
        .collect::<Vec<&str>>()
        .join(",");
    Ok(format!("{}", ingredients))
}

fn split_ingredient_allergen(s: &str) -> (HashSet<&str>, HashSet<&str>) {
    let mut parts = s.split(" (contains ");
    let ingredients: HashSet<_> = parts
        .next()
        .unwrap()
        .split_whitespace()
        .filter(|x| x != &"")
        .collect();
    let allergens: HashSet<_> = parts
        .next()
        .unwrap_or("")
        .trim_end_matches(")")
        .split_whitespace()
        .map(|x| x.trim_end_matches(","))
        .filter(|a| a != &"")
        .collect();
    (ingredients, allergens)
}

fn parse_data(s: &str) -> (Vec<HashSet<&str>>, HashMap<&str, &str>) {
    let mut candidates = HashMap::new();
    let mut ingredient_list = Vec::new();
    for line in s.lines() {
        let (ingredients, allergens) = split_ingredient_allergen(&line);
        for a in &allergens {
            let set = candidates
                .entry(a.clone())
                .or_insert_with(|| ingredients.clone());
            *set = &*set & &ingredients;
        }
        ingredient_list.push(ingredients.clone());
    }

    let mut allergen_map = HashMap::new();
    while let Some((&a, _)) = candidates.iter().find(|(_, s)| s.len() == 1) {
        let &i = candidates[a].iter().next().unwrap();
        allergen_map.insert(a, i);
        for (_, s) in &mut candidates {
            s.remove(&i);
        }
    }

    (ingredient_list, allergen_map)
}

#[allow(dead_code)]
const SAMPLE: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
