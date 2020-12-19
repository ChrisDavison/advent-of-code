use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn day19() -> Result<()> {
    let data = std::fs::read_to_string("input/19.in")?;
    let data2 = std::fs::read_to_string("input/19-2.in")?;
    // let data2 = std::fs::read_to_string("input/19-2.sample")?;

    println!("AoC2020 19.1 -> {}", find_matches(&data)?);
    println!("AoC2020 19.2 -> {}", find_matches(&data2)?);

    Ok(())
}

fn find_matches(data: &str) -> Result<String> {
    let (mut rules, messages) = parse_data(&data);

    loop {
        let to_process = rules
            .iter()
            .filter(|(_k, v)| !terminates(v))
            .map(|(k, _)| k.to_string())
            .collect::<HashSet<String>>();

        if to_process.is_empty() {
            break;
        }

        for rulenum in &to_process {
            let rule = rules[rulenum].to_string();
            let mut new: Vec<String> = Vec::new();
            for part in rule.split(" ") {
                if let Ok(_n) = part.parse::<i64>() {
                    let s = format!("( {} )", rules[part].to_string());
                    new.push(s);
                } else {
                    new.push(part.to_string());
                }
            }
            rules.insert(rulenum.to_string(), new.join(" "));
        }
    }

    let re = Regex::new(&format!("\\b{}\\b", rules["0"].replace(" ", ""))).unwrap();

    Ok(format!(
        "{}",
        messages.iter().filter(|m| re.is_match(m)).count()
    ))
}

fn parse_data<'a>(data: &'a str) -> (HashMap<String, String>, Vec<&'a str>) {
    let mut parts = data.split("\n\n");

    let mut rules = HashMap::new();
    for line in parts.next().expect("No rules given").lines() {
        let mut lineparts = line.split(':');
        let num = lineparts.next().unwrap();
        rules.insert(
            num.to_string(),
            lineparts
                .next()
                .unwrap()
                .replace("\"", "")
                .trim_start_matches(" ")
                .to_string(),
        );
    }

    let messages = parts.next().expect("No messages given").lines().collect();
    (rules, messages)
}

fn terminates(pattern: &str) -> bool {
    pattern.chars().all(|c| "(  )|ab".contains(c))
}
