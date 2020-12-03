use std::collections::HashMap;

pub fn count_letters<'a>(lines: &str) -> HashMap<char, usize> {
    let mut counter = HashMap::new();
    // for line in lines
    for element in lines.chars() {
        if let Some(x) = counter.get_mut(&element) {
            *x += 1;
        } else {
            counter.insert(element, 1);
        }
    }
    counter
}
