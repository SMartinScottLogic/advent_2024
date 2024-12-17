use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
};
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
            if let Some(c) = register_regex.captures(&line) {
                let r = c.name("register").unwrap().as_str();
                let v = c.name("value").unwrap().as_str().parse().unwrap();
                solution.add_register(r.to_string(), v);
            } else if let Some(c) = program_regex.captures(&line) {
                let program = c
                    .name("program")
                    .unwrap()
                    .as_str()
                    .split(",")
                    .map(|v| v.parse().unwrap())
                    .collect();
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
        let r = outputs
            .iter()
            .map(|v| format!("{}", *v))
            .collect::<Vec<_>>()
            .join(",");
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        dump_program(&self.program);

        let mut r = 0;
        if let Some(c) = solve(&self.program, &self.registers) {
            let mut registers = self.registers.clone();
            registers.insert("A".to_string(), c);
            let output = run_program(&self.program, &registers);
            assert_eq!(output, self.program);
            r = c;
        }
        // Implement for problem
        Ok(format!("{}", r))
    }
}

fn dump_program(program: &[u64]) {
    for ((ip1, opcode), (_, operand)) in program.iter().enumerate().tuples() {
        let op = match opcode {
            0 => "adv",
            1 => "bxl",
            2 => "bst",
            3 => "jnz",
            4 => "bxc",
            5 => "out",
            6 => "bdv",
            7 => "cdv",
            _ => panic!(),
        };
        debug!(p = format!("{}: {} {} | {} {}", ip1, op, operand, opcode, operand));
    }
}

fn solve(program: &[u64], registers: &HashMap<String, u64>) -> Option<u64> {
    let mut aim = program.to_vec();
    aim.reverse();
    let mut queue = (0..8).collect::<HashSet<_>>();
    let mut valid = Vec::new();
    for (index, target) in aim.iter().enumerate() {
        valid.clear();
        debug!(index, total = program.len(), queue = queue.len());

        for value in queue {
            let mut local_registers = registers.clone();
            local_registers.insert("A".to_string(), value);
            let out = run_program(program, &local_registers);
            if !out.is_empty() && out.len() == index + 1 && out[0] == *target {
                valid.push(value);
                debug!(?out, ?program);
            }
        }
        let mut next = HashSet::new();
        for v in &valid {
            for i in 0..8 {
                next.insert(v * 8 + i);
            }
        }
        queue = next;
    }
    debug!(?valid);
    valid.iter().cloned().min()
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
            _ => todo!(),
        };
        match opcode {
            // adv
            0 => {
                let numerator = *registers.get("A").unwrap() as f64;
                let denominator = 2_f64.powf(combo_operand as f64);

                let r = (numerator / denominator).floor() as u64;
                registers.insert("A".to_string(), r);
            }
            //bxl
            1 => {
                let lhs = *registers.get("B").unwrap();
                let rhs = operand;
                let r = lhs ^ rhs;
                registers.insert("B".to_string(), r);
            }
            //bst
            2 => {
                let r = combo_operand % 8;
                registers.insert("B".to_string(), r);
            }
            //jnz
            3 => {
                let a = *registers.get("A").unwrap();
                if a == 0 {
                } else {
                    ip = operand as usize;
                    increment_ip = false;
                }
            }
            //bxc
            4 => {
                let lhs = *registers.get("B").unwrap();
                let rhs = *registers.get("C").unwrap();
                let r = lhs ^ rhs;
                registers.insert("B".to_string(), r);
            }
            //out
            5 => {
                let r = combo_operand % 8;
                outputs.push(r);
            }
            //bdv
            6 => {
                let numerator = *registers.get("A").unwrap() as f64;
                let denominator = 2_f64.powf(combo_operand as f64);

                let r = (numerator / denominator).floor() as u64;
                registers.insert("B".to_string(), r);
            }
            //cdv
            7 => {
                let numerator = *registers.get("A").unwrap() as f64;
                let denominator = 2_f64.powf(combo_operand as f64);

                let r = (numerator / denominator).floor() as u64;
                registers.insert("C".to_string(), r);
            }
            _ => todo!(),
        }
        if increment_ip {
            ip += 2;
        }
    }
    debug!(output=?outputs);
    outputs
}
