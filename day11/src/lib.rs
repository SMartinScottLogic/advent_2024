use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    stones: Vec<ResultType>,
    part1_answer: ResultType,
    part2_answer: ResultType,
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
    fn analyse(&mut self, _is_full: bool) {
        self.part1_answer = num_stones(&self.stones, 25);
        self.part2_answer = num_stones(&self.stones, 75);
    }

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        Ok(self.part1_answer)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        Ok(self.part2_answer)
    }
}

fn num_stones(original_stones: &[ResultType], steps: usize) -> ResultType {
    let mut stones: HashMap<ResultType, ResultType> = HashMap::new();
    for stone in original_stones {
        *stones.entry(*stone).or_default() += 1;
    }
    debug!(i = 0, ?stones);
    for i in 0..steps {
        let mut new_stones = HashMap::new();
        for (stone, count) in stones {
            let ns = if stone == 0 {
                vec![(1, count)]
            } else if has_even_digits(stone) {
                let (l, r) = split(stone);
                vec![(l, count), (r, count)]
            } else {
                vec![(stone * 2024, count)]
            };
            for (s, c) in ns {
                *new_stones.entry(s).or_default() += c;
            }
        }
        stones = new_stones.clone();
        debug!(i = i + 1, ?stones);
    }
    let r = stones.values().sum();
    // Implement for problem
    r
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
