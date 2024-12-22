use memoize::memoize;
use std::{
    cmp::min,
    collections::VecDeque,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    codes: Vec<String>,
}
impl Solution {
    fn add_code(&mut self, code: String) {
        self.codes.push(code);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            solution.add_code(line);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let r = self
            .codes
            .iter()
            .map(|code| {
                let len = cheapest_sequence_len(code, 3);
                let numeric_part: usize = code.strip_suffix("A").unwrap().parse().unwrap();
                debug!(?code, ?len, ?numeric_part);
                len * numeric_part
            })
            .fold(0 as ResultType, |acc, v| acc + v as ResultType);

        // Implement for problem
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let r = self
            .codes
            .iter()
            .map(|code| {
                let len = cheapest_sequence_len(code, 26);
                let numeric_part: usize = code.strip_suffix("A").unwrap().parse().unwrap();
                debug!(?code, ?len, ?numeric_part);
                len * numeric_part
            })
            .fold(0 as ResultType, |acc, v| acc + v as ResultType);

        // Implement for problem
        Ok(r)
    }
}

fn cheapest_sequence_len(code: &str, num_keypads: usize) -> usize {
    let number_pad: &[u8; 12] = b"789456123X0A";
    let mut px = 2;
    let mut py = 3;
    let mut total = 0;

    for c in code.bytes() {
        for ty in 0..4 {
            for tx in 0..3 {
                if number_pad[ty * 3 + tx] == c {
                    total += cheapest_pad_len(px, py, tx, ty, num_keypads);
                    px = tx;
                    py = ty;
                }
            }
        }
    }
    total
}

fn cheapest_pad_len(px: usize, py: usize, tx: usize, ty: usize, num_keypads: usize) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((px, py, "".to_string()));
    let mut cheapest = usize::MAX;

    while let Some(v) = queue.pop_front() {
        if v.1 == ty && v.0 == tx {
            let cost = cheapest_robot(&(v.2 + "A"), num_keypads);
            cheapest = min(cheapest, cost);
            continue;
        }
        if v.0 == 0 && v.1 == 3 {
            // Illegal - don't generate key presses
            continue;
        }
        match v.1.cmp(&ty) {
            std::cmp::Ordering::Less => queue.push_back((v.0, v.1 + 1, v.2.clone() + "v")),
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => queue.push_back((v.0, v.1 - 1, v.2.clone() + "^")),
        }
        match v.0.cmp(&tx) {
            std::cmp::Ordering::Less => queue.push_back((v.0 + 1, v.1, v.2.clone() + ">")),
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => queue.push_back((v.0 - 1, v.1, v.2.clone() + "<")),
        }
    }
    cheapest
}

fn cheapest_robot(presses: &str, num_robots: usize) -> usize {
    if num_robots == 1 {
        return presses.len();
    }

    let mut total = 0;
    let pad = b"X^A<v>";

    let mut px = 2;
    let mut py = 0;

    for c in presses.bytes() {
        for ty in 0..2 {
            for tx in 0..3 {
                if pad[ty * 3 + tx] == c {
                    total += cheapest_dir_pad(px, py, tx, ty, num_robots);
                    px = tx;
                    py = ty;
                }
            }
        }
    }
    total
}

#[memoize]
fn cheapest_dir_pad(px: usize, py: usize, tx: usize, ty: usize, num_robots: usize) -> usize {
    let mut cheapest = usize::MAX;
    let mut queue = VecDeque::new();
    queue.push_back((px, py, "".to_string()));

    while let Some(v) = queue.pop_front() {
        if v.0 == tx && v.1 == ty {
            let cost = cheapest_robot(&(v.2.clone() + "A"), num_robots - 1);
            if cost < cheapest {
                cheapest = cost;
            }
            continue;
        }
        if v.0 == 0 && v.1 == 0 {
            // Illegal - don't generate key presses
            continue;
        }
        match v.1.cmp(&ty) {
            std::cmp::Ordering::Less => queue.push_back((v.0, v.1 + 1, v.2.clone() + "v")),
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => queue.push_back((v.0, v.1 - 1, v.2.clone() + "^")),
        }
        match v.0.cmp(&tx) {
            std::cmp::Ordering::Less => queue.push_back((v.0 + 1, v.1, v.2.clone() + ">")),
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => queue.push_back((v.0 - 1, v.1, v.2.clone() + "<")),
        }
    }
    cheapest
}
