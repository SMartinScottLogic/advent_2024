use std::{collections::HashSet, io::{BufRead, BufReader}};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::{Direction, Grid, Point};

pub type ResultType = u64;

#[derive(Debug)]
enum Decision {
    Step,
    Turn,
}
#[derive(Debug, Default)]
pub struct Solution {
    grid: Grid<char, isize>,
}
impl Solution {
    fn set(&mut self, x: usize, y: usize, c: char) {
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
                solution.set(x, y, c);
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
        let mut steps = 0;
        let mut guard_pos = self.find_guard();
        let mut direction = Direction::N;
        let mut visited = HashSet::new();
        loop {
            visited.insert((guard_pos.x(), guard_pos.y()));
            info!(steps, ?guard_pos, ?direction, "stage");
            let front_pos = match direction {
                Direction::N => guard_pos.north(),
                Direction::E => guard_pos.east(),
                Direction::S => guard_pos.south(),
                Direction::W => guard_pos.west(),
                _ => panic!("unexpected direction {:?}", direction)
            };
            match match self.grid.get(&front_pos) {
                Some('.') => Decision::Step,
                Some('#') => Decision::Turn,
                // Guard can't stand in front of themselves
                Some('^') => Decision::Step,
                Some(c) => panic!("Unknown entry in grid: {}", c),
                None => break,
            } {
                Decision::Step => {
                    steps += 1;
                    guard_pos = front_pos;
                },
                Decision::Turn => {
                    direction = match direction {
                        Direction::N => Direction::E,
                        Direction::E => Direction::S,
                        Direction::S => Direction::W,
                        Direction::W => Direction::N,
                        _ => panic!("unexpected direction {:?}", direction)        
                    }
                }
            }
        }
        Ok(visited.len() as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok(0)
    }
}

impl Solution {
    fn find_guard(&self) -> Point<isize> {
        // find position of guard
        for y in self.grid.dimensions().y.clone() {
            for x in self.grid.dimensions().x.clone() {
                if let Some('^') = self.grid.get(&Point::new(x, y)) {
                    return Point::new(x, y);
                }
            }
        }
    panic!("No guard!");
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
