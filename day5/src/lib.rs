use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    rules: HashSet<String>,
    updates: Vec<Vec<ResultType>>,
}
impl Solution {
    fn add_rule(&mut self, rule: String) {
        self.rules.insert(rule);
    }

    fn add_update(&mut self, update: Vec<ResultType>) {
        self.updates.push(update);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut stage = 0;
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            if line.trim().is_empty() {
                stage += 1;
                continue;
            }
            if stage == 0 {
                solution.add_rule(line.to_string());
            }
            if stage == 1 {
                let update = line.split(',').map(|v| v.parse().unwrap()).collect();
                solution.add_update(update);
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut total = 0;
        for update in &self.updates {
            let correct = self.is_correct(update);
            debug!(correct, "correct");
            if correct {
                let mid = update.get(update.len() / 2).unwrap();
                debug!(mid, ?update, "mid");
                total += mid;
            }
        }
        // Implement for problem
        Ok(total)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut total = 0;
        for update in &self.updates {
            let correct = self.is_correct(update);
            debug!(correct, "correct");
            if !correct {
                let mut fixed = update.clone();
                self.fix(&mut fixed);
                let mid = fixed.get(update.len() / 2).unwrap();
                debug!(?update, ?fixed, mid, "mid");
                total += mid;
            }
        }
        // Implement for problem
        Ok(total)
    }
}

impl Solution {
    fn is_correct(&self, update: &[ResultType]) -> bool {
        for (i, page) in update.iter().enumerate() {
            for j in 0..i {
                let probe = update.get(j).unwrap();
                let v = format!("{}|{}", page, probe);
                if self.rules.contains(&v) {
                    return false;
                }
            }
        }
        true
    }

    fn fix(&self, arr: &mut Vec<ResultType>) {
        let mut swapped = true;

        while swapped {
            swapped = false;
            for i in 0..arr.len() - 1 {
                let v = format!("{}|{}", arr[i + 1], arr[i]);
                if self.rules.contains(&v) {
                    debug!(
                        "volation in {:?}: {} after {}: {}",
                        arr,
                        arr[i + 1],
                        arr[i],
                        v
                    );
                    arr.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
    }
}
