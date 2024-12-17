use std::{collections::HashMap, io::{BufRead, BufReader}};
use regex::Regex;
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = String;

#[derive(Debug, Default)]
pub struct Solution {
    registers: HashMap<String, u64>,
    program: Vec<u64>,
}
impl Solution {
    fn add_register(&mut self, register: String, value: u64) {
        self.registers.insert(register, value);
    }
    fn set_program(&mut self, program: Vec<u64>) {
        self.program = program;
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut register_regex = Regex::new(r"^Register (?<register>.): (?<value>\d+)$").unwrap();
        let mut program_regex = Regex::new(r"^Program: (?<program>.*$)$").unwrap();
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            if let Some(c)  = register_regex.captures(&line) {
                let r = c.name("register").unwrap().as_str();
                let v = c.name("value").unwrap().as_str().parse().unwrap();
                solution.add_register(r.to_string(), v);
            } else if let Some(c) = program_regex.captures(&line) {
                let program = c.name("program").unwrap().as_str().split(",").map(|v| v.parse().unwrap()).collect();
                solution.set_program(program)
            } else {
                info!("unprocessed: {}", line);
           }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let outputs = run_program(&self.program, &self.registers);
        let r = outputs.iter()
        .map(|v| format!("{}", *v))
        .collect::<Vec<_>>().join(",");
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok("".to_string())
    }
}

    fn run_program(program: &[u64], registers: &HashMap<String, u64>) -> Vec<u64> {
        // Implement for problem
        let mut registers = registers.clone();
        let mut ip = 0_usize;

        let mut outputs = Vec::new();

        loop {
            if ip >= program.len() {
                break;
            }
            let mut increment_ip = true;
            let opcode = program[ip];
            let operand = program[ip + 1];
            let combo_operand = match operand {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => *registers.get("A").unwrap(),
                5 => *registers.get("B").unwrap(),
                6 => *registers.get("C").unwrap(),
                _ => todo!()
            };
            match opcode {
                0 => {
                    let numerator = *registers.get("A").unwrap() as f64;
                    let denominator = 2_f64.powf(combo_operand as f64);

                    let r = (numerator / denominator).floor() as u64;
                    registers.insert("A".to_string(), r);
                },
                1 => {
                    let lhs = *registers.get("B").unwrap();
                    let rhs = operand;
                    let r = lhs ^ rhs;
                    registers.insert("B".to_string(), r);
                }
                2 => {
                    let r = combo_operand % 8;
                    registers.insert("B".to_string(), r);
                }
                3 => {
                    let a = *registers.get("A").unwrap();
                    if a == 0 {} else {
                        ip = operand as usize;
                        increment_ip = false;
                    }
                }
                4 => {
                    let lhs = *registers.get("B").unwrap();
                    let rhs = *registers.get("C").unwrap();
                    let r = lhs ^ rhs;
                    registers.insert("B".to_string(), r);
                }
                5 => {
                    let r = combo_operand % 8;
                    outputs.push(r);
                }
                6 => {
                    let numerator = *registers.get("A").unwrap() as f64;
                    let denominator = 2_f64.powf(combo_operand as f64);

                    let r = (numerator / denominator).floor() as u64;
                    registers.insert("B".to_string(), r);
                }
                7 => {
                    let numerator = *registers.get("A").unwrap() as f64;
                    let denominator = 2_f64.powf(combo_operand as f64);

                    let r = (numerator / denominator).floor() as u64;
                    registers.insert("C".to_string(), r);
                }
                _ => todo!()
            }
            if increment_ip {
                ip += 2;
            }
        }
        info!(output=?outputs);
        outputs
    }
