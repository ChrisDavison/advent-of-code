use std::collections::HashMap;

pub fn count_letters<'a>(lines: &str) -> HashMap<char, usize> {
    let mut counter = HashMap::new();
    for letter in lines.chars() {
        let count = counter.entry(letter).or_insert(0);
        *count += 1;
    }
    counter
}
