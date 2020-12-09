use anyhow::Result;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

const DAY: usize = 4;

#[allow(dead_code)]
#[derive(Error, Debug)]
enum ParsePassportError {
    #[error("Passport had an empty field")]
    NoField,
    #[error("Passport had an empty value")]
    NoValue,
}

pub fn day04() -> Result<()> {
    let data = std::fs::read_to_string(format!("input/day{}.in", DAY))?;
    let passports: Vec<_> = data
        .split("\n\n")
        .map(parse_passport)
        .filter_map(|x| Some(x.ok()?))
        .collect();
    let n_valid_passports = passports
        .iter()
        .filter(|passport| passport_validator::part1(passport))
        .count();
    let n_valid_passports2 = passports
        .iter()
        .filter(|passport| passport_validator::part2(passport))
        .count();
    println!("2020 {}-1 -> {}", DAY, n_valid_passports);
    println!("2020 {}-2 -> {}", DAY, n_valid_passports2);
    Ok(())
}

fn parse_passport(s: &str) -> Result<HashMap<&str, &str>> {
    let mut fields: HashMap<&str, &str> = HashMap::new();
    for entry in s.split_whitespace() {
        let mut parts = entry.split(':');
        let field = parts.next().ok_or(ParsePassportError::NoField)?;
        let value = parts.next().ok_or(ParsePassportError::NoValue)?;
        if !field.is_empty() {
            fields.insert(field, value);
        }
    }
    Ok(fields)
}

mod passport_validator {
    use super::*;
    // cid (Country ID) - ignored, missing or not.
    type FieldValidator = dyn Fn(&str) -> bool;
    const VALIDATORS: &[(&str, &FieldValidator)] = &[
        ("hgt", &is_valid_height),
        ("pid", &is_valid_pid),
        ("eyr", &is_valid_eyr),
        ("byr", &is_valid_byr),
        ("iyr", &is_valid_iyr),
        ("ecl", &is_valid_ecl),
        ("hcl", &is_valid_hcl),
    ];

    pub fn part1(fields: &HashMap<&str, &str>) -> bool {
        let necessary: HashSet<_> = ["hgt", "pid", "eyr", "iyr", "ecl", "hcl", "byr"]
            .iter()
            .cloned()
            .collect();
        necessary.is_subset(&fields.keys().copied().collect())
    }

    pub fn part2(fields: &HashMap<&str, &str>) -> bool {
        VALIDATORS
            .iter()
            .all(|(key, validate_func)| match fields.get(*key) {
                Some(value) => validate_func(value),
                _ => false,
            })
    }

    pub fn is_valid_height(f: &str) -> bool {
        // hgt (Height) - a number followed by either cm or in:
        //     If cm, the number must be at least 150 and at most 193.
        //     If in, the number must be at least 59 and at most 76.
        match Some(f.split_at(f.len() - 2))
            .and_then(|(val, unit)| Some((val.parse::<i64>().ok()?, unit)))
        {
            Some((150..=193, "cm")) => true,
            Some((59..=76, "in")) => true,
            _ => false,
        }
    }

    // pid (Passport ID) - 9 digit number, with leading zeros
    pub fn is_valid_pid(f: &str) -> bool {
        f.chars().filter(|c| c.is_digit(10)).count() == 9
    }

    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    pub fn is_valid_eyr(f: &str) -> bool {
        matches!(f.parse::<i64>().ok(), Some(2020..=2030))
    }

    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    pub fn is_valid_iyr(f: &str) -> bool {
        matches!(f.parse::<i64>().ok(), Some(2010..=2020))
    }

    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    pub fn is_valid_byr(f: &str) -> bool {
        matches!(f.parse::<i64>().ok(), Some(1920..=2002))
    }

    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pub fn is_valid_ecl(f: &str) -> bool {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&f)
    }

    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    pub fn is_valid_hcl(f: &str) -> bool {
        match f.strip_prefix('#') {
            Some(hex) => hex.chars().filter(|c| c.is_digit(16)).count() == 6,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        let tests = vec![(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm",
            true,
        )];
        for (test, expected) in tests {
            assert_eq!(
                passport_validator::part1(&parse_passport(test).unwrap()),
                expected
            );
        }
    }

    #[test]
    fn hcl() {
        let tests = vec![
            ("#abcdef", true),
            ("#abcdeg", false),
            ("#abc123", true),
            ("#abcdeff", false),
        ];
        for (input, expected) in tests {
            assert_eq!(passport_validator::is_valid_hcl(input), expected);
        }
    }

    #[test]
    fn ecl() {
        let tests = vec![
            ("amb", true),
            ("blu", true),
            ("brn", true),
            ("gry", true),
            ("grn", true),
            ("hzl", true),
            ("oth", true),
            ("oth", true),
            ("askdjaksjd", false),
            ("123", false),
        ];
        for (input, expected) in tests {
            println!("{} {}", input, expected);
            assert_eq!(passport_validator::is_valid_ecl(input), expected);
        }
    }

    #[test]
    fn byr() {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        let tests = vec![
            ("1919", false),
            ("abc", false),
            ("1920", true),
            ("2002", true),
            ("2003", false),
        ];
        for (input, expected) in tests {
            assert_eq!(passport_validator::is_valid_byr(input), expected);
        }
    }

    #[test]
    fn eyr() {
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        let tests = vec![
            ("2019", false),
            ("abc", false),
            ("2020", true),
            ("2030", true),
            ("2031", false),
        ];
        for (input, expected) in tests {
            assert_eq!(passport_validator::is_valid_eyr(input), expected);
        }
    }

    #[test]
    fn iyr() {
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        let tests = vec![
            ("2009", false),
            ("abc", false),
            ("2010", true),
            ("2020", true),
            ("2021", false),
        ];
        for (input, expected) in tests {
            assert_eq!(passport_validator::is_valid_iyr(input), expected);
        }
    }

    #[test]
    fn pid() {
        let tests = vec![
            ("123456789", true),
            ("000456789", true),
            ("a23456789", false),
            ("123458", false),
            ("", false),
        ];
        for (input, expected) in tests {
            assert_eq!(passport_validator::is_valid_pid(input), expected);
        }
    }

    #[test]
    fn hgt() {
        let tests = vec![
            // Test cm
            ("149cm", false),
            ("150cm", true),
            ("193cm", true),
            ("194cm", false),
            ("194cma", false),
            // Test in
            ("58in", false),
            ("59in", true),
            ("76in", true),
            ("77in", false),
            ("77ina", false),
            // Test other
            ("190", false),
            ("abc", false),
        ];
        for (input, expected) in tests {
            assert_eq!(passport_validator::is_valid_height(input), expected);
        }
    }

    #[test]
    fn random() {
        assert_eq!('\n'.is_whitespace(), true);
    }
}
