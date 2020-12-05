use crate::part::Part;
use anyhow::Result;
use std::collections::{HashMap, HashSet};

pub fn day4(data: &[&str], part: Part) -> Result<()> {
    let mut valid = 0;
    let mut fields: HashMap<&str, &str> = HashMap::new();
    let validator = match part {
        Part::One => passport_rule::part1,
        Part::Two => passport_rule::part2,
    };
    for (i, line) in data.iter().enumerate() {
        let line = line.trim();
        if line.is_empty() || i == data.len() - 1 {
            if validator(&fields) {
                valid += 1;
            }
            fields.clear();
            continue;
        }
        for entry in line.trim().split(" ") {
            let parts: Vec<&str> = entry.split(":").collect();
            let field = parts[0];
            let value = parts[1];
            if !field.is_empty() {
                fields.insert(field, value);
            }
        }
    }
    println!("4.{} - {}", part, valid);
    Ok(())
}

mod passport_rule {
    use super::*;
    // cid (Country ID) - ignored, missing or not.
    pub fn part1(fields: &HashMap<&str, &str>) -> bool {
        let necessary: HashSet<&str> = ["hgt", "pid", "eyr", "iyr", "ecl", "hcl", "byr"]
            .iter()
            .cloned()
            .collect();
        let fields: HashSet<&str> = fields.keys().cloned().collect();
        necessary.is_subset(&fields)
    }

    pub fn part2(fields: &HashMap<&str, &str>) -> bool {
        if !fields.contains_key("eyr") || !eyr(fields["eyr"]) {
            false
        } else if !fields.contains_key("iyr") || !iyr(fields["iyr"]) {
            false
        } else if !fields.contains_key("byr") || !byr(fields["byr"]) {
            false
        } else if !fields.contains_key("hgt") || !hgt(fields["hgt"]) {
            false
        } else if !fields.contains_key("pid") || !pid(fields["pid"]) {
            false
        } else if !fields.contains_key("ecl") || !ecl(fields["ecl"]) {
            false
        } else if !fields.contains_key("hcl") || !hcl(fields["hcl"]) {
            false
        } else {
            true
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
        match year_check(f, 2020, 2030) {
            Ok(b) => b,
            Err(e) => false,
        }
    }

    pub fn iyr(f: &str) -> bool {
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        match year_check(f, 2010, 2020) {
            Ok(b) => b,
            Err(e) => false,
        }
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
        match year_check(f, 1920, 2002) {
            Ok(b) => b,
            Err(e) => false,
        }
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
    fn test_day4_hcl() {
        assert_eq!(passport_rule::hcl("#abcdef"), true);
        assert_eq!(passport_rule::hcl("#abcdeg"), false);
        assert_eq!(passport_rule::hcl("#abc123"), true);
        assert_eq!(passport_rule::hcl("#abcdeff"), false);
    }

    #[test]
    fn test_day4_ecl() {
        assert_eq!(passport_rule::ecl("amb"), true);
        assert_eq!(passport_rule::ecl("blu"), true);
        assert_eq!(passport_rule::ecl("brn"), true);
        assert_eq!(passport_rule::ecl("gry"), true);
        assert_eq!(passport_rule::ecl("grn"), true);
        assert_eq!(passport_rule::ecl("hzl"), true);
        assert_eq!(passport_rule::ecl("oth"), true);
        assert_eq!(passport_rule::ecl("oth"), true);
        assert_eq!(passport_rule::ecl("askdjaksjd"), false);
        assert_eq!(passport_rule::ecl("123"), false);
    }

    #[test]
    fn test_day4_byr() {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        assert_eq!(passport_rule::byr("1919"), false);
        assert_eq!(passport_rule::byr("abc"), false);
        assert_eq!(passport_rule::byr("1920"), true);
        assert_eq!(passport_rule::byr("2002"), true);
        assert_eq!(passport_rule::byr("2003"), false);
    }

    #[test]
    fn test_day4_eyr() {
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        assert_eq!(passport_rule::eyr("2019"), false);
        assert_eq!(passport_rule::eyr("abc"), false);
        assert_eq!(passport_rule::eyr("2020"), true);
        assert_eq!(passport_rule::eyr("2030"), true);
        assert_eq!(passport_rule::eyr("2031"), false);
    }

    #[test]
    fn test_day4_iyr() {
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        assert_eq!(passport_rule::iyr("2009"), false);
        assert_eq!(passport_rule::iyr("abc"), false);
        assert_eq!(passport_rule::iyr("2010"), true);
        assert_eq!(passport_rule::iyr("2020"), true);
        assert_eq!(passport_rule::iyr("2021"), false);
    }

    #[test]
    fn test_day4_pid() {
        assert_eq!(passport_rule::pid("123456789"), true);
        assert_eq!(passport_rule::pid("000456789"), true);
        assert_eq!(passport_rule::pid("a23456789"), false);
        assert_eq!(passport_rule::pid("123458"), false);
        assert_eq!(passport_rule::pid(""), false);
    }

    #[test]
    fn test_day4_hgt() {
        // Test cm
        assert_eq!(passport_rule::hgt("149cm"), false);
        assert_eq!(passport_rule::hgt("150cm"), true);
        assert_eq!(passport_rule::hgt("193cm"), true);
        assert_eq!(passport_rule::hgt("194cm"), false);
        assert_eq!(passport_rule::hgt("194cma"), false);
        // Test in
        assert_eq!(passport_rule::hgt("58in"), false);
        assert_eq!(passport_rule::hgt("59in"), true);
        assert_eq!(passport_rule::hgt("76in"), true);
        assert_eq!(passport_rule::hgt("77in"), false);
        assert_eq!(passport_rule::hgt("77ina"), false);
        // Test other
        assert_eq!(passport_rule::hgt("190"), false);
        assert_eq!(passport_rule::hgt("abc"), false);
        assert_eq!(passport_rule::hgt(""), false);
    }
}
