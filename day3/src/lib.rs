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
            let mut i = 0;
            loop {
                let c = regex.captures(&input[i..]);
                if c.is_none() {
                    break;
                }
                let c = c.unwrap();
                debug!("empty: {}", c.get(0).unwrap().as_str());
                let a: ResultType = c.name("a").unwrap().as_str().parse().unwrap();
                let b: ResultType = c.name("b").unwrap().as_str().parse().unwrap();
                total += a * b;
                debug!("{} x {} = {} => {}", a, b, a * b, total);
                i += input[i..].find(c.get(0).unwrap().as_str()).unwrap();
                i += c.get(0).unwrap().as_str().len();
                debug!("next: {}", &input[i..]);
            }
        }
        // Implement for problem
        Ok(total)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let regex = Regex::new(r"do\(\)|don't\(\)|mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").unwrap();
        let mut total = 0;
        let mut enable = true;
        for input in &self.inputs {
            let mut i = 0;
            loop {
                let c = regex.find_at(input, i);
                if c.is_none() {
                    break;
                }
                let end = c.unwrap().end();
                let c = c.unwrap();
                debug!("condition: {}", c.as_str());
                match c.as_str() {
                    "do()" => enable = true,
                    "don't()" => enable = false,
                    other => {
                        let c = regex.captures(other);
                        if c.is_none() {
                            break;
                        }
                        let c = c.unwrap();
                        debug!("empty: {}", c.get(0).unwrap().as_str());
                        let a: ResultType = c.name("a").unwrap().as_str().parse().unwrap();
                        let b: ResultType = c.name("b").unwrap().as_str().parse().unwrap();
                        if enable {
                            total += a * b;
                        }
                        debug!("{} x {} = {} => {}", a, b, a * b, total);
                    }
                }
                i = end;
            }
        }
        // Implement for problem
        Ok(total)
    }
}
