use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = i64;

#[derive(Debug, Default)]
pub struct Solution {
    initials: Vec<ResultType>,
}
impl Solution {
    fn add_initial(&mut self, initial: ResultType) {
        self.initials.push(initial);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let initial = line.parse().unwrap();
            solution.add_initial(initial);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut total = 0;
        for initial in &self.initials {
            let mut v = *initial;
            for _ in 1..=2000 {
                v = next_secret(v);
                debug!(?v, initial);
            }
            debug!(?v, initial);
            total += v;
        }
        // Implement for problem
        Ok(total)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut bananas: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
        for initial in &self.initials {
            let mut values = Vec::new();
            let mut v = *initial;
            for _ in 1..=2000 {
                v = next_secret(v);
                values.push(v);
            }
            // comvert to prices
            for v in values.iter_mut() {
                *v %= 10;
            }
            let mut seen = HashSet::new();
            for (a, b, c, d, e) in values.iter().tuple_windows() {
                let key = (b - a, c - b, d - c, e - d);
                if seen.insert(key) {
                    *bananas.entry(key).or_default() += *e;
                }
            }
        }
        let max = bananas.values().max().unwrap();
        // Implement for problem
        Ok(*max)
    }
}

fn next_secret(initial: i64) -> i64 {
    let mut i = prune(mix(initial, initial * 64));
    i = prune(mix(i, i / 32));
    i = prune(mix(i, i * 2048));
    i
}

fn mix(i: i64, v: i64) -> i64 {
    v ^ i
}

fn prune(v: i64) -> i64 {
    v % 16777216
}
