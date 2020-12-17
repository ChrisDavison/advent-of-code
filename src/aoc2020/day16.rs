use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};

pub fn day16() -> Result<()> {
    let data = std::fs::read_to_string("input/16.in")?;

    let (rules, my_ticket, other_tickets) = parse_data(&data)?;
    let (mut bad_fields, mut valid_tickets) = (Vec::new(), Vec::new());
    other_tickets
        .iter() // iter over Tickets
        .map(|t| (t, invalid_fields(&rules, t))) // (ticket, Vec<usize>)
        .for_each(|(t, invalid_fields)| {
            if invalid_fields.is_empty() {
                // push good tickets into 'valid tickets'
                valid_tickets.push(t.clone());
            } else {
                // push fields from bad tickets into bad fields
                bad_fields.extend(invalid_fields);
            }
        });

    part1(&bad_fields)?;
    part2(&rules, my_ticket, &valid_tickets)?;
    Ok(())
}

type RuleSet = HashMap<String, Vec<(usize, usize)>>;
type Ticket = Vec<usize>;

fn part1(bad_fields: &[usize]) -> Result<()> {
    println!("AoC2020 16.1 -> {}", bad_fields.iter().sum::<usize>());
    Ok(())
}

fn part2(rules: &RuleSet, my: Ticket, tickets: &[Ticket]) -> Result<()> {
    let product_of_my_departure_columns: usize = my
        .iter()
        .zip(find_rule_order(&rules, &tickets).iter())
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
                if !rule_matches(&rules[*rule_name], *value) {
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

    // unwrap should be safe, as we loop above until all columns have a single
    // associated rule
    possible
        .iter()
        .map(|x| x.iter().next().unwrap().to_string())
        .collect()
}

fn invalid_fields(rs: &RuleSet, t: &Ticket) -> Vec<usize> {
    t.iter()
        .filter(|v| !rs.values().any(|rule| rule_matches(&rule, **v)))
        .copied()
        .collect()
}

fn parse_rule(s: &str) -> Result<(String, Vec<(usize, usize)>)> {
    let mut parts = s.split(':');
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
    Ok((name.to_string(), values))
}

fn parse_ticket(s: &str) -> Vec<usize> {
    s.split(',').filter_map(|x| Some(x.parse().ok()?)).collect()
}

fn parse_data(data: &str) -> Result<(RuleSet, Ticket, Vec<Ticket>)> {
    let mut rules: HashMap<String, Vec<(usize, usize)>> = HashMap::new();
    let mut my_ticket: Ticket = Vec::new();
    let mut other_tickets: Vec<Ticket> = Vec::new();
    let mut state = SM::ParseRules;

    for line in data.lines() {
        let line = line.trim();
        if line == "" {
            state.next();
            continue;
        }
        if line.contains("ticket") {
            continue;
        }
        match state {
            SM::ParseRules => {
                let (name, values) = parse_rule(line)?;
                rules.insert(name, values);
            }
            SM::ParseMyTicket => {
                my_ticket = parse_ticket(line);
            }
            SM::ParseOtherTickets => other_tickets.push(parse_ticket(line)),
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

#[inline(always)]
fn rule_matches(rule: &[(usize, usize)], num: usize) -> bool {
    for range in rule {
        if (range.0..=range.1).contains(&num) {
            return true;
        }
    }
    false
}
