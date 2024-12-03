use regex::Regex;
use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    inputs: Vec<String>,
}
impl Solution {
    fn add_input(&mut self, input: &str) {
        self.inputs.push(input.to_string());
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            solution.add_input(&line);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let regex = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").unwrap();
        let mut total = 0;
        for input in &self.inputs {
            for capture in regex.captures_iter(input) {
                let a: ResultType = capture.name("a").unwrap().as_str().parse().unwrap();
                let b: ResultType = capture.name("b").unwrap().as_str().parse().unwrap();
                total += a * b;
            }
        }
        Ok(total)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let regex = Regex::new(r"(?<op>do|don't|mul)\(((?<a>\d{1,3}),(?<b>\d{1,3}))?\)").unwrap();
        let mut total = 0;
        let mut enable = true;
        for input in &self.inputs {
            for capture in regex.captures_iter(input) {
                let op = capture.name("op").unwrap().as_str();
                match op {
                    "do" => enable = true,
                    "don't" => enable = false,
                    "mul" => {
                        if enable {
                            let a: ResultType =
                                capture.name("a").unwrap().as_str().parse().unwrap();
                            let b: ResultType =
                                capture.name("b").unwrap().as_str().parse().unwrap();
                            total += a * b;
                        }
                    }
                    _ => panic!("unknown operator '{op}'"),
                }
            }
        }
        Ok(total)
    }
}
