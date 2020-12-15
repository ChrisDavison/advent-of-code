use anyhow::Result;

pub fn day09() -> Result<()> {
    let data = std::fs::read_to_string("input/09.in")?;
    let mut s = Solver::new(&data, 25);
    s.part_1();
    s.part_2();
    Ok(())
}

struct Solver {
    result: Option<i64>,
    window: usize,
    data: Vec<i64>,
}

impl Solver {
    fn new(data: &str, window: usize) -> Solver {
        Solver {
            result: None,
            window,
            data: data
                .lines()
                .map(|x| x.trim())
                .filter_map(|x| Some(x.parse::<i64>().ok()?))
                .collect(),
        }
    }

    fn part_1(&mut self) {
        let mut idx = self.window; // Start from an index that will fill buffer
        let mut found: Option<i64> = None;
        while let Some(n) = self.data.get(idx) {
            let history = &self.data[idx - self.window..idx];
            let mut found_a_pair = false;
            for (i, val) in history.iter().enumerate() {
                let need = n - val;
                if history[i..].contains(&need) {
                    found_a_pair = true;
                    break;
                }
            }
            if !found_a_pair {
                found = Some(*n);
                break;
            }
            idx += 1;
        }
        match found {
            Some(num) => {
                self.result = found;
                println!("2020 9.1 -> {}", num);
            }
            None => eprintln!("2020 9.1 ERROR. No num found."),
        }
    }

    fn part_2(&mut self) {
        if self.result.is_none() {
            self.part_1();
        }

        let target = match self.result {
            Some(t) => t,
            None => {
                eprintln!("Exiting, as part 1 unsuccessful.");
                return;
            }
        };
        'outer: for window_size in 2..self.data.len() - 1 {
            for idx in 0..self.data.len() - window_size {
                let mut buffer: Vec<i64> = self.data[idx..idx + window_size].to_vec();
                buffer.sort_unstable();
                if buffer.iter().sum::<i64>() == target {
                    self.result = Some(buffer[0] + buffer[window_size - 1]);
                    break 'outer;
                }
            }
        }
        match self.result {
            Some(num) => println!("2020 9.2 -> {}", num),
            None => eprintln!("2020 9.2 ERROR. No num found."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";
        dbg!(input);
        let mut s = Solver::new(input, 5);
        s.part_1();
        assert_eq!(s.result, Some(127));
    }

    #[test]
    fn example_part2() {
        let input = "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";
        dbg!(input);
        let mut s = Solver::new(input, 5);
        s.part_2();
        assert_eq!(s.result, Some(62));
    }
}
