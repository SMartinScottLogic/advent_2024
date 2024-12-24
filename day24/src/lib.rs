use std::{collections::{HashMap, HashSet}, io::{BufRead, BufReader}};
use itertools::Itertools;
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug)]
struct Wire {
    lhs: String,
    op: String,
    rhs: String,
    out: String,
}

#[derive(Debug, Default)]
pub struct Solution {
    initial_values: HashMap<String, ResultType>,
    gates: Vec<Wire>,
}
impl Solution {
    fn add_initial(&mut self, wire: &str, initial_value: ResultType) {
        self.initial_values.insert(wire.to_string(), initial_value);
    }

    fn add_gate(&mut self, lhs: &str, op: &str, rhs: &str, out: &str) {
        self.gates.push(Wire { lhs: lhs.to_string(), op: op.to_string(), rhs: rhs.to_string(), out: out.to_string() })
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let regex = regex::Regex::new(r"^(?<lhs>[^\s]+) (?<op>[^\s]+) (?<rhs>[^\s]+) -> (?<out>.*)$").unwrap();
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            if line.trim().is_empty() {
                continue;
            }
            match line.split_once(':') {
                Some((a, b)) => {
                    let wire = a.trim();
                    let initial_value = b.trim().parse().unwrap();
                    solution.add_initial(wire, initial_value);
                },
                None => {
                    let c = regex.captures(&line).unwrap();
                    let lhs = c.name("lhs").unwrap().as_str();
                    let op = c.name("op").unwrap().as_str();
                    let rhs = c.name("rhs").unwrap().as_str();
                    let out = c.name("out").unwrap().as_str();
                    solution.add_gate(lhs, op, rhs, out);
                }
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut values = self.initial_values.clone();
        let outputs = self.gates.iter().flat_map(|w| [w.lhs.clone(), w.rhs.clone(), w.out.clone()].into_iter()).filter(|w| w.starts_with("z")).collect::<HashSet<_>>();
        loop {
            for gate in &self.gates {
                if values.contains_key(&gate.lhs) && values.contains_key(&gate.rhs) {
                    let lhs = values[&gate.lhs];
                    let rhs = values[&gate.rhs];
                    let r = match gate.op.as_str() {
                        "AND" => {
                            if lhs != 0 && rhs != 0 {1} else {0}
                        },
                        "OR" => {
                            if lhs != 0 || rhs !=0 {1} else {0}
                        },
                        "XOR" => {
                            if lhs != rhs {1} else {0}
                        },
                        op => panic!("Unexpected operation: {}", op),
                    };
                    values.insert(gate.out.clone(), r);
                }
            }
            if outputs.iter().all(|o| values.contains_key(o)) {
                break;
            }
        }
        debug!(?values);
        let r = values.iter()
        .filter(|(name, _value)| name.starts_with("z"))
        .sorted_by_cached_key(|(name, _value)| (*name).clone())
        .rev()
        .fold(0, |mut acc, (name, value)|{
            debug!(?name, ?value, ?acc);
            acc *= 2;
            acc += value;
            acc
        });
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
