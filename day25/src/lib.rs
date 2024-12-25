use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    locks: HashMap<usize, Vec<String>>,
}
impl Solution {
    fn add_lock_row(&mut self, id: usize, line: String) {
        self.locks.entry(id).or_default().push(line);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        let mut lock_id = 0;
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            if line.trim().is_empty() {
                lock_id += 1;
                continue;
            }
            solution.add_lock_row(lock_id, line);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut locks = Vec::new();
        let mut keys = Vec::new();

        for s in self.locks.values() {
            if s[0].starts_with('#') {
                locks.push(s);
            } else {
                keys.push(s);
            }
        }

        debug!(?keys, ?locks);
        let mut r = 0;
        for lock in &locks {
            for key in &keys {
                let overlaps = lock.iter().zip(key.iter()).any(|(lock_r, key_r)| {
                    key_r
                        .chars()
                        .zip(lock_r.chars())
                        .any(|(a, b)| a == '#' && b == '#')
                });
                r += if overlaps { 0 } else { 1 }
            }
        }
        // Implement for problem
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    use tracing_test::traced_test;
    use utils::Solution;

    #[test]
    #[traced_test]
    fn read() {
        let input = "replace for problem";
        let r = BufReader::new(input.as_bytes());
        let s = crate::Solution::try_from(r).unwrap();
        assert_eq!(0 as ResultType, s.answer_part1(false).unwrap());
    }
}
