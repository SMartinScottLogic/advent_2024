use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};
use tracing::debug;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    left: Vec<u64>,
    right: Vec<u64>,
}
impl Solution {
    fn distance(a: &ResultType, b: &ResultType) -> ResultType {
        if a < b {
            b - a
        } else {
            a - b
        }
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            let mut i = line.split_whitespace().map(|s| s.parse::<u64>().unwrap());
            let left = i.next().unwrap();
            let right = i.next().unwrap();
            solution.left.push(left);
            solution.right.push(right);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut left = self.left.clone();
        left.sort();
        let mut right = self.right.clone();
        right.sort();
        for (a, b) in left.iter().zip(right.iter()) {
            debug!(a, b, "sorted");
        }
        let answer = left
            .iter()
            .zip(right.iter())
            .map(|(a, b)| Self::distance(a, b))
            .sum();
        // Implement for problem
        Ok(answer)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let right_count = self.right.iter().fold(HashMap::new(), |mut acc, value| {
            let entry: &mut ResultType = acc.entry(*value).or_default();
            *entry += 1;
            acc
        });

        let answer = self
            .left
            .iter()
            .map(|v| v * right_count.get(v).cloned().unwrap_or_default())
            .sum();

        // Implement for problem
        Ok(answer)
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
