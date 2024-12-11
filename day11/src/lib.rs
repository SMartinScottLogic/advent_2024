use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    stones: Vec<ResultType>,
}
impl Solution {
    fn set_stones(&mut self, stones: Vec<ResultType>) {
        self.stones = stones;
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let input = line
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            solution.set_stones(input);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut stones = self.stones.clone();
        info!(i = 0, ?stones);
        for i in 0..25 {
            let mut new_stones = Vec::new();
            for stone in stones {
                if stone == 0 {
                    new_stones.push(1);
                } else if has_even_digits(stone) {
                    let (l, r) = split(stone);
                    new_stones.push(l);
                    new_stones.push(r);
                } else {
                    new_stones.push(stone * 2024);
                }
            }
            stones = new_stones;
            info!(i = i + 1, ?stones);
        }
        // Implement for problem
        Ok(stones.len() as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut stones = self.stones.clone();
        debug!(i = 0, ?stones);
        for i in 0..10 {
            let mut new_stones = Vec::new();
            for stone in stones {
                if stone == 0 {
                    new_stones.push(1);
                } else if has_even_digits(stone) {
                    let (l, r) = split(stone);
                    new_stones.push(l);
                    new_stones.push(r);
                } else {
                    new_stones.push(stone * 2024);
                }
            }
            stones = new_stones;
            info!(i = i + 1, ?stones);
            debug!(i = i + 1, num_stones = stones.len());
        }
        // Implement for problem
        Ok(stones.len() as ResultType)
    }
}

fn has_even_digits(v: ResultType) -> bool {
    format!("{}", v).len() % 2 == 0
}

fn split(v: ResultType) -> (ResultType, ResultType) {
    let i = format!("{}", v);
    let s = i.len() / 2;
    let (a, b) = i.split_at(s);
    (a.to_owned().parse().unwrap(), b.to_owned().parse().unwrap())
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
