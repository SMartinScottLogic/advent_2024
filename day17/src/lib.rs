use std::{collections::HashMap, io::{BufRead, BufReader}};
use itertools::Itertools;
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
        dump_program(&self.program);
        
        let mut v = 0;
        let r = loop {
            let mut registers = self.registers.clone();
            registers.insert("A".to_string(), v);
            let output = run_program(&self.program, &registers);
            info!(v, ?output);
            if output == self.program {
                break v;
            }
            v += 1;
            if v > 10 {
                break v;
            }
        };
        solve(&self.program, &self.registers);
        // Implement for problem
        Ok(format!("{}", r))
    }
}

fn dump_program(program: &[u64]) {
    for ((ip1, opcode), (ip2, operand)) in program.iter().enumerate().tuples() {
        let op = match opcode {
            0 => "div",
            1 => "bxl",
            2 => "bst",
            3 => "jnz",
            4 => "bxc",
            5 => "out",
            6 => "bdv",
            7 => "cdv",
            _ => panic!()
        };
        info!(p = format!("{}: {} {} | {} {}", ip1, op, operand, opcode, operand));
    }
}

/*
opt = Optimize()
s = BitVec('s', 64)
a, b, c = s, 0, 0
for x in [2,4,1,5,7,5,4,3,1,6,0,3,5,5,3,0]:
    b = a % 8
    b = b ^ 5
    c = a / (1 << b)
    b = b ^ c
    b = b ^ 6
    a = a / (1 << 3)
    opt.add((b % 8) == x)
opt.add(a == 0)
opt.minimize(s)
assert str(opt.check()) == 'sat'
print(opt.model().eval(s))

    values = [2, 4, 1, 1, 7, 5, 1, 5, 0, 3, 4, 3, 5, 5, 3, 0][::-1]
    queue = list(range(8))

    for index in range(len(values)):
        print(f"index: {index+1} of {len(values)}; queue: {len(queue)}")
        valid = []

        for value in queue:
            a = value
            # 2,4
            b = a % 8
            # 1,1
            b = b ^ 1
            # 7,5
            c = a // (1 << b)
            # 1,5
            b = b ^ 5
            # 0,3 a = a // 8
            # 4,3
            b = b ^ c
            # 5,5 print b % 8
            if b % 8 == values[index]:
                valid.append(value)
            # 3,0 jnz ignored
        next_level = []
        for v in valid:
            for i in range(8):
                next_level.append(v * 8 + i)

        queue = next_level

    print(min(valid))
*/

fn solve(program: &[u64], registers: &HashMap<String, u64>) {
    let mut queue = (0..8).collect::<Vec<_>>();
    let mut valid = Vec::new();
    for index in 0..program.len() {
        info!(index, total=program.len(), queue=queue.len());

        for value in queue {
            let mut local_registers = registers.clone();
            local_registers.insert("A".to_string(), value);
            let out = run_program(program, &local_registers);
            if !out.is_empty() && out[0]==program[index] {
                valid.push(value);
            }
        }
        let mut next = Vec::new();
        while let Some(v) = valid.pop() {
            for i in 0..8 {
                next.push(v * 8 + i);
            }
        }
        queue = next;
    }
    info!(?valid);
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
        debug!(output=?outputs);
        outputs
    }
