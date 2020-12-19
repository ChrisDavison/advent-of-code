#![allow(dead_code, unused_variables, unused_mut, unreachable_code)]
use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::HashMap;

pub fn day19() -> Result<()> {
    let filename = "input/19.in";
    //let filename = "input/19.sample";
    let data = std::fs::read_to_string(filename)?;
    let (mut rules, mut endpoints, messages) = parse_data(&data);
    while endpoints.len() < rules.keys().count() {
        for (rulenum, rule) in rules.clone() {
            if endpoints.iter().find(|s| **s == rulenum).is_some() {
                continue;
            }
            let new = rule
                .split(" ")
                .map(|c| {
                    if endpoints.contains(&c.clone().to_string()) {
                        format!("({})", rules[&c.clone().to_string()].to_string())
                    } else {
                        c.clone().to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join(" ");
            //println!("NO | #{} > {}  --->  {}", rulenum, rule, new);
            rules.insert(rulenum.to_string(), new.clone());
            if new.chars().all(|c| !c.is_digit(10)) {
                endpoints.push(rulenum);
            }
        }
    }

    let tidied = rules["0"].replace(" ", "");
    let tidied = tidied.replace("(a)", "a");
    let tidied = tidied.replace("(b)", "b");
    let re = Regex::new(&format!("\\b{}\\b", tidied)).unwrap();
    let mut count = 0;
    for msg in messages {
        if re.is_match(msg) {
            println!("{}", msg);
            count += 1;
        }
    }

    println!("{}", count);

    println!("AoC2020 19.1 -> {}", part1(&data)?);
    println!("AoC2020 19.2 -> {}", part1(&data)?);
    Ok(())
}

fn part1(data: &str) -> Result<String> {
    return Err(anyhow!("Part 1 not implemented"));
    Ok(format!("{}", 0))
}

fn part2(data: &str) -> Result<String> {
    return Err(anyhow!("Part 2 not implemented"));
    Ok(format!("{}", 0))
}

#[derive(Debug, Clone, PartialEq)]
enum Rule {
    Recur(Vec<String>),
    Letter(char),
}

fn expand_rules(rules: HashMap<String, Vec<Rule>>) {
    for k in rules.keys() {
        //let mut new = Vec::new();
        for subrule in &rules[k] {
            // if matches!(rulenum, Rule::Letter(_)) {
            //     continue;
            // }
            // if let Rule::Letter(_) = rulenum {
            //     continue;
            // }
            println!("{} {:?}", k, subrule);
        }
        println!();
        // rules[k] = new;
    }
}

fn parse_data<'a>(data: &'a str) -> (HashMap<String, String>, Vec<String>, Vec<&'a str>) {
    let mut parts = data.split("\n\n");

    let mut rules = HashMap::new();
    let mut messages = Vec::new();
    //let mut endpoints = Vec::new();
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

    let endpoints: Vec<String> = rules
        .iter()
        .filter(|(num, rule)| rule.chars().all(|c| !c.is_digit(10)))
        .map(|(k, _)| k.to_string())
        .collect();

    //println!("ENDPOINTS {:?}", endpoints);

    for line in parts.next().expect("No messages given").lines() {
        messages.push(line);
    }

    (rules, endpoints, messages)
}
