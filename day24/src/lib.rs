use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = String;

#[derive(Debug, Clone)]
struct Wire {
    lhs: String,
    op: String,
    rhs: String,
    out: String,
}

#[derive(Debug, Default)]
pub struct Solution {
    initial_values: HashMap<String, u64>,
    gates: Vec<Wire>,
}
impl Solution {
    fn add_initial(&mut self, wire: &str, initial_value: u64) {
        self.initial_values.insert(wire.to_string(), initial_value);
    }

    fn add_gate(&mut self, lhs: &str, op: &str, rhs: &str, out: &str) {
        self.gates.push(Wire {
            lhs: lhs.to_string(),
            op: op.to_string(),
            rhs: rhs.to_string(),
            out: out.to_string(),
        })
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let regex =
            regex::Regex::new(r"^(?<lhs>[^\s]+) (?<op>[^\s]+) (?<rhs>[^\s]+) -> (?<out>.*)$")
                .unwrap();
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
                }
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
        let r = run_network(self.initial_values.clone(), self.gates.clone());
        // Implement for problem
        Ok(format!("{}", r.unwrap()))
    }

    fn answer_part2(&self, is_full: bool) -> Self::Result {
        if !is_full {
            return Ok("".to_string());
        }
        let num_input_bits = self
            .initial_values
            .iter()
            .filter(|(name, _value)| name.starts_with("y"))
            .count();
        /*
         * FULL ADDER
         * (first bits aren't a full adder)
         * (for last FA, COUT is the extra output)
         *
         * A    XOR B    -> VAL0     <= FAGate0
         * A    AND B    -> VAL1     <= FAGate1
         * VAL0 AND CIN  -> VAL2     <= FAGate2
         * VAL0 XOR CIN  -> SUM      <= FAGate3
         * VAL1 OR  VAL2 -> COUT     <= FAGate4
         */
        let mut issues = HashSet::new();
        for gate in &self.gates {
            // FAGate0
            if gate.op == "XOR" && (gate.lhs.starts_with('x') || gate.rhs.starts_with('x')) {
                if gate.lhs == "x00" || gate.rhs == "x00" {
                    if gate.out != "z00" {
                        issues.insert(gate.out.clone());
                    }
                } else if gate.out.starts_with("z") {
                    issues.insert(gate.out.clone());
                }
            }
            // FAGate3
            if gate.op == "XOR"
                && !gate.lhs.starts_with('x')
                && !gate.rhs.starts_with('x')
                && !gate.out.starts_with('z')
            {
                issues.insert(gate.out.clone());
            }

            if gate.out.starts_with('z') {
                if gate.out == format!("z{:02}", num_input_bits) {
                    if gate.op != "OR" {
                        issues.insert(gate.out.clone());
                    }
                } else if gate.op != "XOR" {
                    issues.insert(gate.out.clone());
                }
            }
        }
        let mut to_check = Vec::new();
        for gate in &self.gates {
            if gate.op == "XOR" && (gate.lhs.starts_with('x') || gate.rhs.starts_with('x')) {
                if issues.contains(&gate.out) {
                    continue;
                }
                // HA output - valid
                if gate.out == "z00" {
                    continue;
                }
                // Find FAGate3(s) with this as an input
                let m = self
                    .gates
                    .iter()
                    .filter(|g| g.op == "XOR" && !g.lhs.starts_with('x') && !g.rhs.starts_with('x'))
                    .filter(|g| g.lhs == gate.out || g.rhs == gate.out)
                    .count();
                if m == 0 {
                    issues.insert(gate.out.clone());
                    to_check.push(gate.clone());
                }
            }
        }
        debug!(?issues);

        debug!(?to_check);
        for gate in to_check.iter() {
            // Find all gates outputing to the
            // Find FAGate3 outputing to the corresponding 'z'
            let m = self
                .gates
                .iter()
                .filter(|g| g.op == "XOR" && !g.lhs.starts_with('x') && !g.rhs.starts_with('x'))
                .filter(|g| g.out == format!("z{:02}", &gate.lhs[1..]))
                .collect::<Vec<_>>();
            debug!(?m);
            assert_eq!(1, m.len());
            let m = m.first().unwrap();
            //one of these should come from an OR gate
            let or_matches = self
                .gates
                .iter()
                .filter(|g| g.op == "OR")
                .filter(|g| m.lhs == g.out || m.rhs == g.out)
                .collect::<Vec<_>>();
            debug!(?or_matches);
            assert_eq!(1, or_matches.len());
            let or_match_output = or_matches.first().unwrap().out.clone();
            if m.lhs != or_match_output {
                issues.insert(m.lhs.clone());
            } else if m.rhs != or_match_output {
                issues.insert(m.rhs.clone());
            } else {
                panic!("{} not in {:?}", or_match_output, m);
            }
        }
        debug!(?issues);
        assert_eq!(8, issues.len());
        let r = issues.iter().sorted().join(",");

        Ok(r)
    }
}

fn run_network(mut values: HashMap<String, u64>, gates: Vec<Wire>) -> Option<u64> {
    let outputs = gates
        .iter()
        .flat_map(|w| [w.lhs.clone(), w.rhs.clone(), w.out.clone()].into_iter())
        .filter(|w| w.starts_with("z"))
        .collect::<HashSet<_>>();
    loop {
        let mut changed = false;
        for gate in &gates {
            if values.contains_key(&gate.lhs) && values.contains_key(&gate.rhs) {
                let lhs = values[&gate.lhs];
                let rhs = values[&gate.rhs];
                let r = match gate.op.as_str() {
                    "AND" => {
                        if lhs != 0 && rhs != 0 {
                            1
                        } else {
                            0
                        }
                    }
                    "OR" => {
                        if lhs != 0 || rhs != 0 {
                            1
                        } else {
                            0
                        }
                    }
                    "XOR" => {
                        if lhs != rhs {
                            1
                        } else {
                            0
                        }
                    }
                    op => panic!("Unexpected operation: {}", op),
                };
                if values.insert(gate.out.clone(), r).is_none() {
                    changed = true;
                };
            }
        }
        if outputs.iter().all(|o| values.contains_key(o)) {
            break;
        }
        if !changed {
            return None;
        }
    }
    debug!(?values);
    let r = values
        .iter()
        .filter(|(name, _value)| name.starts_with("z"))
        .sorted_by_cached_key(|(name, _value)| (*name).clone())
        .rev()
        .fold(0, |mut acc, (name, value)| {
            debug!(?name, ?value, ?acc);
            acc *= 2;
            acc += value;
            acc
        });
    Some(r)
}
