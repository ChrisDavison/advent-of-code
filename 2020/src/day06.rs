use anyhow::Result;
use std::collections::*;

pub fn day06(data: &str) -> Result<()> {
    let data: Vec<_> = data.split("\n\n").collect();
    let result = count_answers(&data, count_questions_anyone_answered)
        .iter()
        .sum::<usize>();
    let result2 = count_answers(&data, count_questions_everyone_answered)
        .iter()
        .sum::<usize>();
    println!("2020 6-1 -> {}", result);
    println!("2020 6-2 -> {}", result2);
    Ok(())
}

fn count_answers(s: &[&str], counter: fn(&&str) -> usize) -> Vec<usize> {
    s.iter().map(counter).collect()
}

fn count_questions_anyone_answered(s: &&str) -> usize {
    // Count Nkeys
    s.chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<HashSet<char>>()
        .len()
}

fn count_questions_everyone_answered(s: &&str) -> usize {
    // Count Nentries where value == n_rows
    let mut answer_map: HashMap<char, usize> = HashMap::new();
    for letter in s.chars() {
        if letter == '\n' {
            continue;
        }
        let e = answer_map.entry(letter).or_insert(0);
        *e += 1;
    }
    let n_rows = count_number_of_groups(s);
    answer_map.iter().filter(|(_, &v)| v == n_rows).count()
}

fn count_number_of_groups(s: &str) -> usize {
    s.lines().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        let ex1 = "abcx\nabcy\nabcz";
        assert_eq!(
            count_answers(ex1, count_questions_anyone_answered)
                .iter()
                .sum::<usize>(),
            6
        );

        let ex2 = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
        assert_eq!(
            count_answers(ex2, count_questions_anyone_answered)
                .iter()
                .sum::<usize>(),
            11
        );
    }

    #[test]
    fn examples_part2() {
        let ex2 = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
        assert_eq!(
            count_answers(ex2, count_questions_everyone_answered)
                .iter()
                .sum::<usize>(),
            6
        );
    }

    #[test]
    fn example_questions_anyone_answered() {
        let inputs = vec![
            ("abc", 3),
            ("a\nb\nc", 3),
            ("ab\nac", 3),
            ("a\na\na\na", 1),
            ("b", 1),
        ];
        for (input, expected) in inputs {
            assert_eq!(count_questions_anyone_answered(input), expected);
        }
    }

    #[test]
    fn example_questions_everyone_answered() {
        let inputs = vec![
            ("abc", 3),
            ("a\nb\nc", 0),
            ("ab\nac", 1),
            ("a\na\na\na", 1),
            ("b", 1),
        ];
        for (input, expected) in inputs {
            assert_eq!(count_questions_everyone_answered(input), expected);
        }
    }

    #[test]
    fn count_groups() {
        let tests = vec![("abc", 1), ("abc\ndef", 2), ("abc\ndef\ng", 3), ("", 0)];
        for (input, expected) in tests {
            assert_eq!(count_number_of_groups(input), expected);
        }
    }
}
