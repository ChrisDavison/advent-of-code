use aoc2020::*;

pub fn day13() -> Result<String> {
    let data = include_str!("../input/day13");
    let lines: Vec<&str> = data.lines().collect();
    let start: usize = lines[0].parse().expect("Failed to parse start time");
    let timetable: Vec<Option<usize>> = parse_bus_timetable(lines[1]);

    let output = format!(
        "2020 13.1 -> {}\n2020 13.1 -> {}",
        part_1(start, &timetable),
        part_2(&timetable)
    );
    Ok(output)
}

fn parse_bus_timetable(s: &str) -> Vec<Option<usize>> {
    s.split(',')
        .map(|x| {
            if x == "x" {
                None
            } else {
                Some(x.parse().unwrap())
            }
        })
        .collect()
}

fn part_1(start: usize, buses: &[Option<usize>]) -> usize {
    let mut next_afters = Vec::new();
    for &bus in buses {
        if let Some(bus) = bus {
            let mut timetable_iter = (bus..).step_by(bus).skip_while(|&x| x < start);
            let next = timetable_iter.next().unwrap();
            next_afters.push((bus, next));
        }
    }
    next_afters.sort_by(|x, y| x.1.cmp(&y.1));
    let best_bus = next_afters[0];
    let diff = best_bus.1 - start;
    best_bus.0 * diff
}

fn part_2(buses: &[Option<usize>]) -> usize {
    let bus_and_offset = buses
        .iter()
        .enumerate()
        .map(|(i, x)| x.as_ref().map(|b| (i, b)))
        .flatten()
        .collect::<Vec<_>>();
    let mut timestamp = 0;
    let mut step = *bus_and_offset[0].1;
    for (delta, bus_id) in &bus_and_offset[1..] {
        while (timestamp + delta) % **bus_id != 0 {
            timestamp += step;
        }
        step *= **bus_id;
    }
    timestamp
}
