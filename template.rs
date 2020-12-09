use anyhow::Result;
use rayon::prelude::*;

const DAY: usize = XX;

pub fn dayXX() -> Result<()> {
    let data = std::fs::read_to_string(format!("input/day{}.in", DAY))?;
    let mut s = Solver::new(&data, 25);
    s.part_1();
    s.part_2();
    Ok(())
}

struct Solver {
    data: Vec<&str>,
}

impl Solver {
    fn new(data: &str, window: usize) -> Solver {
        Solver {
            data: data
                .as_parallel_string()
                .lines()
                .map(|x| x.trim())
                .filter_map(|x| Some(x.parse::<i64>().ok()?))
                .collect(),
        }
    }

    fn part_1(&mut self) {}

    fn part_2(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_day_was_changed() {
        assert_ne!(DAY, 0);
    }
}
