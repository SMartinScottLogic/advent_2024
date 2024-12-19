use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    patterns: Vec<String>,
    designs: Vec<String>,
}
impl Solution {
    fn set_patterns(&mut self, patterns: Vec<String>) {
        self.patterns = patterns;
    }
    fn add_design(&mut self, design: String) {
        self.designs.push(design);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        let mut stage = 0;
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let line = line.trim();
            if line.is_empty() {
                stage = 1;
                continue;
            }
            if stage == 0 {
                let patterns = line.split(",").map(|p| p.trim().to_string()).collect();
                solution.set_patterns(patterns);
                continue;
            }
            solution.add_design(line.to_string());
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let r = self
            .designs
            .iter()
            .filter(|d| can_make(d, &self.patterns))
            .count() as ResultType;
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let r = self
            .designs
            .iter()
            .map(|d| num_ways(d, &self.patterns, &mut HashMap::new()))
            .sum();
        Ok(r)
    }
}

fn can_make(design: &str, patterns: &Vec<String>) -> bool {
    for p in patterns {
        if design == p {
            return true;
        }
        if design.starts_with(p) && can_make(&design[p.len()..], patterns) {
            return true;
        }
    }
    debug!(design, false);
    false
}

fn num_ways(
    design: &str,
    patterns: &Vec<String>,
    memo: &mut HashMap<String, ResultType>,
) -> ResultType {
    if let Some(value) = memo.get(design) {
        return *value;
    }
    let mut total = 0;
    for p in patterns {
        if design == p {
            total += 1;
        }
        if design.starts_with(p) {
            total += num_ways(&design[p.len()..], patterns, memo);
        }
    }
    memo.insert(design.to_string(), total);
    total
}
