use aoc2020::*;

pub fn day19() -> Result<()> {
    let input = include_str!("../input/day19");
    println!("2020 19.1 -> {}", find_matches(input));
    let input2 = include_str!("../input/day19_2");
    println!("2020 19.2 -> {}", find_matches(input2));

    Ok(())
}

fn find_matches(data: &str) -> String {
    let (mut rules, messages) = parse_data(data);

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
            for part in rule.split(' ') {
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

    format!("{}", messages.iter().filter(|m| re.is_match(m)).count())
}

fn parse_data(data: &str) -> (HashMap<String, String>, Vec<&str>) {
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
                .trim_start_matches(' ')
                .to_string(),
        );
    }

    let messages = parts.next().expect("No messages given").lines().collect();
    (rules, messages)
}

fn terminates(pattern: &str) -> bool {
    pattern.chars().all(|c| "(  )|ab".contains(c))
}

#[allow(dead_code)]
const SAMPLES: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

#[allow(dead_code)]
const SAMPLE2: &str = r#"0: 8 11
1: "a"
2: "b"
8: 9 | 9 8
9: 2 2
11: 1 | 1 11

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
