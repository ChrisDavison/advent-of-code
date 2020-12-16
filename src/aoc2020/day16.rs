#![allow(dead_code, unused_imports, unused_variables, unreachable_code)]
use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{self, Display};

pub fn day16() -> Result<()> {
    let data = std::fs::read_to_string("input/16.in")?;
    // let data = std::fs::read_to_string("input/16.sample")?;

    let (rules, my_ticket, other_tickets) = parse_data(&data)?;
    part1(&rules, &other_tickets)?;
    part2(&rules, &other_tickets)?;
    Ok(())
}

fn part1(rules: &RuleSet, tickets: &[Ticket]) -> Result<()> {
    let mut sum_invalid = 0;
    for ticket in tickets {
        for invalid in invalid_fields(rules, ticket) {
            sum_invalid += invalid;
        }
    }
    println!("AoC2020 16.1 -> {}", sum_invalid);
    Ok(())
}

fn part2(rules: &RuleSet, tickets: &[Ticket]) -> Result<()> {
    let mut valid_tickets = Vec::new();
    for ticket in tickets {
        if invalid_fields(rules, ticket).is_empty() {
            valid_tickets.push(ticket);
        }
    }
    println!(
        "{} tickets, {} are valid",
        tickets.len(),
        valid_tickets.len()
    );

    return Err(anyhow!("Part 2 not implemented!"));
    let mut sum_departures = 0;
    println!("AoC2020 16.2 -> {}", sum_departures);
    Ok(())
}

#[derive(Debug)]
enum SM {
    ParseRules,
    ParseMyTicket,
    ParseOtherTickets,
}

type RuleSet = HashMap<String, Rule>;
type Ticket = Vec<usize>;

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<(usize, usize)>,
}

impl Rule {
    fn is_valid(&self, num: usize) -> bool {
        for range in &self.ranges {
            if (range.0..=range.1).contains(&num) {
                return true;
            }
        }
        false
    }
}

fn invalid_fields(rs: &RuleSet, t: &Ticket) -> Vec<usize> {
    let mut invalid = Vec::new();
    for value in t {
        let mut valid = false;
        for (_, rule) in rs {
            if rule.is_valid(*value) {
                valid = true;
                break;
            }
        }
        if !valid {
            invalid.push(*value);
        }
    }
    // println!("{:?}", invalid);
    invalid
}

impl Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}-{}, {}-{}",
            self.name, self.ranges[0].0, self.ranges[0].1, self.ranges[1].0, self.ranges[1].1
        )
    }
}

fn parse_data(data: &str) -> Result<(RuleSet, Ticket, Vec<Ticket>)> {
    let mut rules: HashMap<String, Rule> = HashMap::new();
    let mut my_ticket: Ticket = Vec::new();
    let mut other_tickets: Vec<Ticket> = Vec::new();
    let mut state = SM::ParseRules;

    for line in data.lines() {
        match state {
            SM::ParseRules => {
                if line.trim() == "" {
                    state = SM::ParseMyTicket;
                    continue;
                }
                let mut parts = line.split(':');
                let name = parts.next().ok_or_else(|| anyhow!("No name for rule"))?;
                let ranges = parts
                    .next()
                    .ok_or_else(|| anyhow!("No ranges for rule"))?
                    .split(" or ")
                    .map(|x| {
                        x.trim()
                            .split('-')
                            .map(|y| y.parse().unwrap())
                            .collect::<Vec<usize>>()
                    })
                    .collect::<Vec<_>>();
                let mut values = Vec::new();
                for range in ranges {
                    values.push((range[0], range[1]));
                }
                rules.insert(
                    name.to_string(),
                    Rule {
                        name: name.to_string(),
                        ranges: values,
                    },
                );
            }
            SM::ParseMyTicket => {
                // my_ticket = line
                //     .split(",")
                //     .filter_map(|x| Some(x.parse().ok()?))
                //     .collect::<Vec<usize>>();
                if line.trim() == "" {
                    state = SM::ParseOtherTickets;
                    continue;
                } else if line.contains("ticket") {
                    continue;
                } else {
                    my_ticket = line
                        .split(',')
                        .filter_map(|x| Some(x.parse().ok()?))
                        .collect();
                }
            }
            SM::ParseOtherTickets => {
                if line.trim() == "" {
                    state = SM::ParseOtherTickets;
                    continue;
                } else if line.contains("ticket") {
                    continue;
                } else {
                    other_tickets.push(
                        line.split(',')
                            .filter_map(|x| Some(x.parse().ok()?))
                            .collect(),
                    )
                }
            }
        }
    }

    Ok((rules, my_ticket, other_tickets))
}
