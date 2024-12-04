use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::{Matrix, Point};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    grid: Matrix<char>,
}
impl Solution {
    fn set_character(&mut self, x: usize, y: usize, c: char) {
        self.grid.set(x as isize, y as isize, c);
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
        for sy in self.grid.min_y()..=self.grid.max_y() {
            for sx in self.grid.min_x()..=self.grid.max_x() {
                if let Some('X') = self.grid.get(sx, sy) {
                    total += self.walk(sx + 1, sy, 1, 0, "X", "XMAS");
                    total += self.walk(sx + 1, sy + 1, 1, 1, "X", "XMAS");
                    total += self.walk(sx, sy + 1, 0, 1, "X", "XMAS");
                    total += self.walk(sx - 1, sy + 1, -1, 1, "X", "XMAS");
                    total += self.walk(sx - 1, sy, -1, 0, "X", "XMAS");
                    total += self.walk(sx - 1, sy - 1, -1, -1, "X", "XMAS");
                    total += self.walk(sx, sy - 1, 0, -1, "X", "XMAS");
                    total += self.walk(sx + 1, sy - 1, 1, -1, "X", "XMAS");
                    if total > 0 {
                        debug!(sx, sy, total, "found");
                    }
                }
            }
        }
        // Implement for problem
        Ok(total)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut total = 0;
        for sy in self.grid.min_y()..=self.grid.max_y() {
            for sx in self.grid.min_x()..=self.grid.max_x() {
                if let Some('M') = self.grid.get(sx, sy) {
                    for (dx, dy, nd) in [
                        (1, -1, [(1, 1), (-1, -1)]),
                        (1, 0, [(0, 1), (0, -1)]), 
                        (1, 1, [(1, -1), (-1, 1)]),
                        (0, 1, [(-1, 0), (1, 0)]),
                        (-1, 1, [(1, 1), (-1, -1)]),
                        (-1, 0, [(0, 1), (0, -1)]),
                        (-1, -1, [(1, -1), (-1, 1)]),
                        (0, -1, [(1, 0), (-1, 0)]),
                        ] {
                        let c1 = self.walk(sx + dx, sy + dy, dx, dy, "M", "MAS");
                        if c1 > 0 {
                            for (ndx, ndy) in nd {
                                let nsx = sx + dx - ndx;
                                let nsy = sy + dy - ndy;
                                if let Some('M') = self.grid.get(nsx, nsy) {
                                    let c2 = self.walk(nsx + ndx, nsy + ndy, ndx, ndy, "M", "MAS");
                                    if c2 > 0 {
                                    info!(sx, sy, dx, dy, c1, nsx, nsy, ndx, ndy, c2, "ns");                                    
                                    total += c1 * c2;
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
    fn walk(&self, x: isize, y: isize, dx: isize, dy: isize, s: &str, target: &str) -> ResultType {
        if let Some(c) = self.grid.get(x, y) {
            if *c == target.chars().nth(s.len()).unwrap() {
                if target.len() == s.len() + 1 {
                    1
                } else {
                    let mut s = s.to_owned();
                    s.push(*c);
                    self.walk(x + dx, y + dy, dx, dy, &s, target)
                }
            } else {
                0
            }
        } else {
            0
        }
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
