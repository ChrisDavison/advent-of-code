use anyhow::{anyhow, Result};
use std::str::FromStr;

const DAY: usize = 4;

enum Height {
    Cm(usize),
    Inch(usize),
}

impl std::fmt::Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Height::Cm(n) => write!(f, "{} Cm", n),
            Height::Inch(n) => write!(f, "{} Inches", n),
        }
    }
}

impl std::fmt::Debug for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Height::Cm(n) => write!(f, "{} Cm", n),
            Height::Inch(n) => write!(f, "{} Inches", n),
        }
    }
}

impl FromStr for Height {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match Some(s.split_at(s.len() - 2))
            .and_then(|(val, unit)| Some((val.parse::<usize>().ok()?, unit)))
        {
            Some((x @ 150..=193, "cm")) => Ok(Height::Cm(x)),
            Some((x @ 59..=76, "in")) => Ok(Height::Inch(x)),
            _ => Err(anyhow!("Bad height")),
        }
    }
}

#[derive(Debug)]
struct Passport {
    ecl: String,
    hcl: String,
    eyr: usize,
    byr: usize,
    iyr: usize,
    hgt: Height,
    pid: String,
    cid: Option<String>,
}

impl FromStr for Passport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if !has_all_fields(s) {
            return Err(anyhow!("Doesn't have all fields"));
        }
        let (mut ecl, mut hcl, mut pid) = (String::new(), String::new(), String::new());
        let mut cid = None;
        let mut hgt: Height = Height::Cm(0);
        let (mut byr, mut iyr, mut eyr): (usize, usize, usize) = (0, 0, 0);
        for entry in s.split_whitespace() {
            let mut parts = entry.split(':');
            let field = parts.next().ok_or_else(|| anyhow!("No field"))?;
            let value = parts.next().ok_or_else(|| anyhow!("No value"))?;
            match field {
                "ecl" => ecl = validate_ecl(value)?,
                "hcl" => hcl = validate_hcl(value)?,
                "eyr" => eyr = validate_eyr(value)?,
                "byr" => byr = validate_byr(value)?,
                "iyr" => iyr = validate_iyr(value)?,
                "hgt" => hgt = validate_hgt(value)?,
                "pid" => pid = validate_pid(value)?,
                "cid" => cid = Some(value.to_string()),
                _ => continue,
            }
        }
        Ok(Passport {
            ecl,
            hcl,
            eyr,
            byr,
            iyr,
            hgt,
            pid,
            cid,
        })
    }
}

pub fn solve() -> anyhow::Result<()> {
    let data = std::fs::read_to_string(format!("input/day{}.txt", DAY))?;
    let n_simple_passports = data
        .split("\n\n")
        .filter(|entry| has_all_fields(entry))
        .count();
    let passports: Vec<Passport> = data
        .split("\n\n")
        .map(|entry| entry.parse())
        .filter_map(|x| Some(x.ok()?))
        .collect();
    println!("2020 {}-1 -> {}", DAY, n_simple_passports);
    println!("2020 {}-2 -> {}", DAY, passports.len());
    Ok(())
}

fn has_all_fields(entry: &str) -> bool {
    ["hgt", "pid", "eyr", "iyr", "ecl", "hcl", "byr"]
        .iter()
        .all(|x| entry.contains(x))
}

//====================================================================
// VALIDATORS
//====================================================================

// hgt (Height) - XXXcm or YYYin, with XXX/YYY in range
// see Height::from_str (impl of FromStr, .parse())
fn validate_hgt(f: &str) -> Result<Height> {
    // Just a wrapper around Height's FromStr
    // for function 'symmetry'
    f.parse()
}

// pid (Passport ID) - 9 digit number, with leading zeros
fn validate_pid(f: &str) -> Result<String> {
    if f.chars().filter(|c| c.is_digit(10)).count() == 9 {
        Ok(f.to_string())
    } else {
        Err(anyhow!("Bad passport ID"))
    }
}

// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
fn validate_eyr(f: &str) -> Result<usize> {
    match f.parse::<usize>() {
        Ok(x @ 2020..=2030) => Ok(x),
        _ => Err(anyhow!("Bad expiration year")),
    }
    //if matches!(f.parse::<i64>().ok(), Some(2020..=2030))
}

// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
fn validate_iyr(f: &str) -> Result<usize> {
    match f.parse::<usize>() {
        Ok(x @ 2010..=2020) => Ok(x),
        _ => Err(anyhow!("Bad issue year")),
    }
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
fn validate_byr(f: &str) -> Result<usize> {
    match f.parse::<usize>() {
        Ok(x @ 1920..=2002) => Ok(x),
        _ => Err(anyhow!("Bad birth year")),
    }
}

// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
fn validate_ecl(f: &str) -> Result<String> {
    if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&f) {
        Ok(f.to_string())
    } else {
        Err(anyhow!("Bad eye colour"))
    }
}

// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
fn validate_hcl(f: &str) -> Result<String> {
    f.strip_prefix('#')
        .filter(|hex| hex.chars().filter(|c| c.is_digit(16)).count() == 6)
        .map(|x| x.to_string())
        .ok_or(anyhow!("Bad hair colour"))
    // match f.strip_prefix('#') {
    //     Some(hex) => {
    //         if hex.chars().filter(|c| c.is_digit(16)).count() == 6 {
    //             Ok(f.to_string())
    //         } else {
    //             Err(anyhow!("Bad hair colour"))
    //         }
    //     }
    //     _ => Err(anyhow!("Bad hair colour")),
    // }
}
