use crate::part::Part;
use anyhow::Result;
use std::collections::{HashMap, HashSet};

pub fn day4(data: &[&str], part: Part) -> Result<()> {
    let validator = match part {
        Part::One => passport_validator::part1,
        Part::Two => passport_validator::part2,
    };
    let joined = data.join("\n");
    let n_valid_passports = joined
        .split("\n\n")
        .map(parse_passport)
        .filter(|passport| validator(passport))
        .count();
    println!("4.{} - {}", part, n_valid_passports);
    Ok(())
}

fn parse_passport(s: &str) -> HashMap<String, String> {
    let mut fields: HashMap<String, String> = HashMap::new();
    for entry in s.replace("\n", " ").split(" ").into_iter() {
        if entry.len() == 0 {
            continue;
        }
        let parts: Vec<&str> = entry.split(":").collect();
        let field = parts[0];
        let value = parts[1];
        if !field.is_empty() {
            fields.insert(field.to_string(), value.to_string());
        }
    }
    fields
}

mod passport_validator {
    use super::*;
    // cid (Country ID) - ignored, missing or not.

    pub fn part1(fields: &HashMap<String, String>) -> bool {
        let necessary: HashSet<_> = ["hgt", "pid", "eyr", "iyr", "ecl", "hcl", "byr"]
            .iter()
            .cloned()
            .map(|x| x.to_string())
            .collect();
        let fields = fields.keys().cloned().collect();
        necessary.is_subset(&fields)
    }

    pub fn part2(fields: &HashMap<String, String>) -> bool {
        if !part1(&fields) {
            false
        } else {
            [
                eyr(&fields["eyr"]),
                iyr(&fields["iyr"]),
                byr(&fields["byr"]),
                hgt(&fields["hgt"]),
                pid(&fields["pid"]),
                ecl(&fields["ecl"]),
                hcl(&fields["hcl"]),
            ]
            .iter()
            .filter(|&x| !x)
            .count()
                == 0
        }
    }

    pub fn hgt(f: &str) -> bool {
        // hgt (Height) - a number followed by either cm or in:
        //     If cm, the number must be at least 150 and at most 193.
        //     If in, the number must be at least 59 and at most 76.
        let chars: Vec<char> = f.chars().collect();
        let mut digits = Vec::new();
        let mut letters = Vec::new();
        for ch in chars {
            if ch.is_digit(10) {
                digits.push(ch);
            } else {
                letters.push(ch);
            }
        }
        let unit: String = letters.iter().collect();
        let (lower, upper) = match unit.as_str() {
            "cm" => (150, 193),
            "in" => (59, 76),
            _ => return false,
        };

        let num: Option<i64> = digits.iter().collect::<String>().parse().ok();
        match num {
            Some(n) => n >= lower && n <= upper,
            None => false,
        }
    }

    pub fn pid(f: &str) -> bool {
        // pid (Passport ID) - 9 digit number, with leading zeros
        f.chars().filter(|c| c.is_digit(10)).count() == 9
    }

    pub fn eyr(f: &str) -> bool {
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        year_check(f, 2020, 2030).unwrap_or(false)
    }

    pub fn iyr(f: &str) -> bool {
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        year_check(f, 2010, 2020).unwrap_or(false)
    }

    pub fn ecl(f: &str) -> bool {
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        let valid: HashSet<&str> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .cloned()
            .collect();
        valid.contains(f)
    }

    pub fn hcl(f: &str) -> bool {
        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        let chars: Vec<char> = f.chars().collect();
        chars[0] == '#' && (chars[1..].iter().filter(|c| c.is_digit(16)).count() == 6)
    }

    pub fn byr(f: &str) -> bool {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        year_check(f, 1920, 2002).unwrap_or(false)
    }

    fn year_check(value: &str, lower: i64, upper: i64) -> Result<bool> {
        let parsed: i64 = value.parse()?;
        Ok(parsed >= lower && parsed <= upper)
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
            assert_eq!(passport_validator::part1(&parse_passport(test)), expected);
        }
    }

    #[test]
    fn hcl() {
        assert_eq!(passport_validator::hcl("#abcdef"), true);
        assert_eq!(passport_validator::hcl("#abcdeg"), false);
        assert_eq!(passport_validator::hcl("#abc123"), true);
        assert_eq!(passport_validator::hcl("#abcdeff"), false);
    }

    #[test]
    fn ecl() {
        assert_eq!(passport_validator::ecl("amb"), true);
        assert_eq!(passport_validator::ecl("blu"), true);
        assert_eq!(passport_validator::ecl("brn"), true);
        assert_eq!(passport_validator::ecl("gry"), true);
        assert_eq!(passport_validator::ecl("grn"), true);
        assert_eq!(passport_validator::ecl("hzl"), true);
        assert_eq!(passport_validator::ecl("oth"), true);
        assert_eq!(passport_validator::ecl("oth"), true);
        assert_eq!(passport_validator::ecl("askdjaksjd"), false);
        assert_eq!(passport_validator::ecl("123"), false);
    }

    #[test]
    fn byr() {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        assert_eq!(passport_validator::byr("1919"), false);
        assert_eq!(passport_validator::byr("abc"), false);
        assert_eq!(passport_validator::byr("1920"), true);
        assert_eq!(passport_validator::byr("2002"), true);
        assert_eq!(passport_validator::byr("2003"), false);
    }

    #[test]
    fn eyr() {
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        assert_eq!(passport_validator::eyr("2019"), false);
        assert_eq!(passport_validator::eyr("abc"), false);
        assert_eq!(passport_validator::eyr("2020"), true);
        assert_eq!(passport_validator::eyr("2030"), true);
        assert_eq!(passport_validator::eyr("2031"), false);
    }

    #[test]
    fn iyr() {
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        assert_eq!(passport_validator::iyr("2009"), false);
        assert_eq!(passport_validator::iyr("abc"), false);
        assert_eq!(passport_validator::iyr("2010"), true);
        assert_eq!(passport_validator::iyr("2020"), true);
        assert_eq!(passport_validator::iyr("2021"), false);
    }

    #[test]
    fn pid() {
        assert_eq!(passport_validator::pid("123456789"), true);
        assert_eq!(passport_validator::pid("000456789"), true);
        assert_eq!(passport_validator::pid("a23456789"), false);
        assert_eq!(passport_validator::pid("123458"), false);
        assert_eq!(passport_validator::pid(""), false);
    }

    #[test]
    fn hgt() {
        // Test cm
        assert_eq!(passport_validator::hgt("149cm"), false);
        assert_eq!(passport_validator::hgt("150cm"), true);
        assert_eq!(passport_validator::hgt("193cm"), true);
        assert_eq!(passport_validator::hgt("194cm"), false);
        assert_eq!(passport_validator::hgt("194cma"), false);
        // Test in
        assert_eq!(passport_validator::hgt("58in"), false);
        assert_eq!(passport_validator::hgt("59in"), true);
        assert_eq!(passport_validator::hgt("76in"), true);
        assert_eq!(passport_validator::hgt("77in"), false);
        assert_eq!(passport_validator::hgt("77ina"), false);
        // Test other
        assert_eq!(passport_validator::hgt("190"), false);
        assert_eq!(passport_validator::hgt("abc"), false);
        assert_eq!(passport_validator::hgt(""), false);
    }
}
