#![allow(dead_code, unused_variables, unused_mut, unreachable_code)]
use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn day19() -> Result<()> {
    let data = std::fs::read_to_string("input/19.in")?;
    println!("AoC2020 19.1 -> {}", part1(&data)?);

    //let data = std::fs::read_to_string("input/19-2.in")?;
    // let data = std::fs::read_to_string("input/19-2.sample")?;
    // println!("AoC2020 19.2 -> {}", part2(&data)?);
    Ok(())
}

fn part1(data: &str) -> Result<String> {
    let (mut rules, messages) = parse_data(&data);

    loop {
        let to_process = rules
            .iter()
            .filter(|(k, v)| !terminates(v))
            .map(|(k, v)| k.to_string())
            .collect::<HashSet<String>>();
        if to_process.is_empty() {
            break;
        }
        for rulenum in to_process {
            let rule = rules[&rulenum].to_string();
            let mut new: Vec<String> = Vec::new();
            for part in rule.split(" ") {
                if let Ok(n) = part.parse::<i64>() {
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

fn part2(data: &str) -> Result<String> {
    let (mut rules, messages) = parse_data(&data);
    let mut endpoints = rules
        .iter()
        .filter(|(k, v)| terminates(v))
        .map(|(k, v)| k.to_string())
        .collect::<Vec<String>>();

    loop {
        //while endpoints.len() < rules.keys().count() {
        let mut to_process = rules
            .iter()
            .filter(|(k, v)| !terminates(v))
            .map(|(k, _)| k.to_string())
            .collect::<HashSet<String>>();
        println!("{:?}", to_process);
        if to_process.is_empty() {
            break;
        }

        for rulenum in &to_process {
            let rule = rules[rulenum].to_string();
            if endpoints.contains(&rulenum) {
                continue;
            }
            let rulenum_spaced = format!(" {} ", rulenum);

            println!(
                "RULE {} === {:?}",
                rulenum,
                rules[rulenum].split(" ").collect::<HashSet<&str>>()
            );
            let new = rule
                .split(" ")
                .map(|c| {
                    if rule.contains(&rulenum_spaced) || rule.ends_with(&format!(" {}", rulenum)) {
                        let mut newrec = rule.clone();
                        for _ in 0..5 {
                            newrec = newrec.replace(rulenum, &format!(" ( {} ) ", &rule));
                        }
                        newrec = newrec.replace(&rulenum_spaced, " ");
                        newrec = newrec.replace(rulenum, " ");

                        newrec.replace("( a )", "a").replace("( b )", "b")
                    } else if endpoints.contains(&c.clone().to_string()) {
                        format!("( {} ) ", rules[&c.clone().to_string()].to_string())
                    } else {
                        c.clone().to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join(" ");
            rules.insert(rulenum.to_string(), new.clone());

            let mut endpoints = rules
                .iter()
                .filter(|(k, v)| terminates(v))
                .map(|(k, v)| k.to_string())
                .collect::<Vec<String>>();
        }
    }

    let re = Regex::new(&format!("\\b{}\\b", rules["0"].replace(" ", ""))).unwrap();

    Ok(format!(
        "{}",
        messages.iter().filter(|m| re.is_match(m)).count()
    ))
}

#[derive(Debug, Clone, PartialEq)]
enum Rule {
    Recur(Vec<String>),
    Letter(char),
}

fn parse_data<'a>(data: &'a str) -> (HashMap<String, String>, Vec<&'a str>) {
    let mut parts = data.split("\n\n");

    let mut rules = HashMap::new();
    let mut messages = Vec::new();
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

    //println!("ENDPOINTS {:?}", endpoints);

    for line in parts.next().expect("No messages given").lines() {
        messages.push(line);
    }

    (rules, messages)
}

fn terminates(pattern: &str) -> bool {
    pattern.chars().all(|c| "(  )|ab".contains(c))
}
