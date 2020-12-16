use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};

pub fn day16() -> Result<()> {
    let data = std::fs::read_to_string("input/16.in")?;

    let (rules, my_ticket, other_tickets) = parse_data(&data)?;
    part1(&rules, &other_tickets)?;
    part2(&rules, my_ticket, &other_tickets)?;
    Ok(())
}

fn part1(rules: &RuleSet, tickets: &[Ticket]) -> Result<()> {
    let result: usize = tickets
        .iter()
        .map(|ticket| invalid_fields(rules, ticket).iter().sum::<usize>())
        .sum();
    println!("AoC2020 16.1 -> {}", result);
    Ok(())
}

fn part2(rules: &RuleSet, my: Ticket, tickets: &[Ticket]) -> Result<()> {
    let mut valid_tickets = Vec::new();
    for ticket in tickets {
        if invalid_fields(rules, ticket).is_empty() {
            valid_tickets.push(ticket.clone());
        }
    }

    let rule_order = find_rule_order(&rules, &valid_tickets);
    let product_of_my_departure_columns: usize = my
        .iter()
        .zip(rule_order.iter())
        .filter(|(_v, name)| name.contains("departure"))
        .map(|(v, _)| v)
        .product();
    println!("AoC2020 16.2 -> {}", product_of_my_departure_columns);
    Ok(())
}

fn find_rule_order(rules: &RuleSet, tickets: &[Ticket]) -> Vec<String> {
    let keys: HashSet<&String> = rules.keys().collect();
    let mut possible: Vec<_> = (0..keys.len()).map(|_| keys.clone()).collect();

    for ticket in tickets {
        for (i, value) in ticket.iter().enumerate() {
            let mut invalid_rule_names = Vec::new();
            for rule_name in &possible[i] {
                if !rules[*rule_name].is_valid(*value) {
                    invalid_rule_names.push(*rule_name);
                }
            }
            for name in invalid_rule_names {
                possible[i].remove(name);
            }
        }
    }

    let mut processed_rules = Vec::new();
    loop {
        let has_1 = possible
            .iter()
            .enumerate()
            .filter(|(i, x)| x.len() == 1 && !processed_rules.contains(i))
            .map(|(i, x)| (i, x.iter().take(1).collect::<Vec<_>>()[0].to_string()))
            .next()
            .unwrap();
        processed_rules.push(has_1.0);
        for (i, item) in possible.iter_mut().enumerate() {
            if i == has_1.0 {
                continue;
            } else {
                item.remove(&has_1.1);
            }
        }
        if possible.iter().all(|x| x.len() == 1) {
            break;
        }
    }

    possible
        .iter()
        .map(|x| x.iter().collect::<Vec<_>>()[0].to_string())
        .collect()
}

fn invalid_fields(rs: &RuleSet, t: &Ticket) -> Vec<usize> {
    t.iter()
        .filter(|v| !rs.values().any(|rule| rule.is_valid(**v)))
        .copied()
        .collect()
}

fn parse_data(data: &str) -> Result<(RuleSet, Ticket, Vec<Ticket>)> {
    let mut rules: HashMap<String, Rule> = HashMap::new();
    let mut my_ticket: Ticket = Vec::new();
    let mut other_tickets: Vec<Ticket> = Vec::new();
    let mut state = SM::ParseRules;

    for line in data.lines() {
        let line = line.trim();
        if line == "" {
            state.next();
            continue;
        }
        match state {
            SM::ParseRules => {
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
                if line.contains("ticket") {
                    continue;
                } else {
                    my_ticket = line
                        .split(',')
                        .filter_map(|x| Some(x.parse().ok()?))
                        .collect();
                }
            }
            SM::ParseOtherTickets => {
                if line.contains("ticket") {
                    continue;
                } else {
                    other_tickets.push(
                        line.split(',')
                            .filter_map(|x| Some(x.parse().ok()?))
                            .collect(),
                    )
                }
            }
            SM::END => {
                break;
            }
        }
    }

    Ok((rules, my_ticket, other_tickets))
}

#[derive(Debug)]
enum SM {
    ParseRules,
    ParseMyTicket,
    ParseOtherTickets,
    END,
}
impl SM {
    fn next(&mut self) {
        *self = match &self {
            SM::ParseRules => SM::ParseMyTicket,
            SM::ParseMyTicket => SM::ParseOtherTickets,
            _ => SM::END,
        }
    }
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
