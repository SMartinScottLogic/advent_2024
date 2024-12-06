#![feature(let_chains)]
use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::{Direction, Point, SparseGrid};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    grid: SparseGrid<char, isize>,
}
impl Solution {
    fn set_character(&mut self, x: usize, y: usize, c: char) {
        self.grid.set(&Point::new(x as isize, y as isize), c);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            for (x, c) in line.chars().enumerate() {
                solution.set_character(x, y, c);
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut total = 0;
        let range = self.grid.dimensions();
        for sy in range.y.clone() {
            for sx in range.x.clone() {
                let pos = Point::new(sx, sy);
                if let Some('X') = self.grid.get(&pos) {
                    for delta in Direction::iter() {
                        let pos = Point::new(sx, sy) + delta;
                        if self.walk(pos, &delta, "X", "XMAS") {
                            total += 1;
                            debug!(sx, sy, total, "found");
                        }
                    }
                }
            }
        }
        // Implement for problem
        Ok(total)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut total = 0;
        let range = self.grid.dimensions();
        for sy in range.y.clone() {
            for sx in range.x.clone() {
                let start = Point::new(sx, sy);
                if let Some('M') = self.grid.get(&start) {
                    for (delta, next_deltas) in [
                        (Direction::NE, [Direction::SE, Direction::NW]),
                        (Direction::SE, [Direction::NE, Direction::SW]),
                        (Direction::SW, [Direction::SE, Direction::NW]),
                        (Direction::NW, [Direction::NE, Direction::SW]),
                    ] {
                        if self.walk(start + delta, &delta, "M", "MAS") {
                            for next_delta in next_deltas {
                                let new_start = start + delta - next_delta;
                                if let Some('M') = self.grid.get(&new_start) {
                                    if self.walk(new_start + next_delta, &next_delta, "M", "MAS") {
                                        total += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // Implement for problem
        Ok(total / 2)
    }
}

impl Solution {
    fn walk(&self, pos: Point<isize>, delta: &Direction, s: &str, target: &str) -> bool {
        if let Some(c) = self.grid.get(&pos)
            && *c == target.chars().nth(s.len()).unwrap()
        {
            if target.len() == s.len() + 1 {
                true
            } else {
                let mut s = s.to_owned();
                s.push(*c);
                self.walk(pos + delta, delta, &s, target)
            }
        } else {
            false
        }
    }
}
