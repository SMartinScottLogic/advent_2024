use std::io::{BufRead, BufReader};
use regex::Regex;
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    machines: Vec<((ResultType, ResultType),(ResultType, ResultType),(ResultType, ResultType),)>,
}
impl Solution {
    fn add_machine(&mut self, button_a: (ResultType, ResultType), button_b: (ResultType, ResultType), prize: (ResultType, ResultType)) {
        self.machines.push((button_a, button_b, prize));
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let button_regex = Regex::new(r"X\+(?<x>\d+), Y\+(?<y>\d+)$").unwrap();
        let prize_regex = Regex::new(r"X=(?<x>\d+), Y=(?<y>\d+)$").unwrap();
        let mut solution = Self::default();

        let mut button_a = None;
        let mut button_b = None;

        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            if line.is_empty() {
                continue;
            }
            let (lhs, rhs) = line.split_once(": ").unwrap();
            debug!(?line, ?lhs, ?rhs);
            if lhs == "Button A" {
                let c = button_regex.captures(rhs).unwrap();
                let x = c.name("x").unwrap().as_str().parse().unwrap();
                let y = c.name("y").unwrap().as_str().parse().unwrap();
                button_a = Some((x, y));
            }
            else if lhs == "Button B" {
                let c = button_regex.captures(rhs).unwrap();
                let x = c.name("x").unwrap().as_str().parse().unwrap();
                let y = c.name("y").unwrap().as_str().parse().unwrap();
                button_b = Some((x, y));
            }
            else if lhs == "Prize" {
                let c = prize_regex.captures(rhs).unwrap();
                let x = c.name("x").unwrap().as_str().parse().unwrap();
                let y = c.name("y").unwrap().as_str().parse().unwrap();
                solution.add_machine(button_a.unwrap(), button_b.unwrap(), (x, y));
                button_a = None;
                button_b = None;
            } else {
                panic!("unexpected line: {}", line);
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        
        // Implement for problem
        Ok(0)
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
