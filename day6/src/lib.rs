use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
};
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
        let (looped, visited) = self.analyse(None);
        assert!(!looped);
        Ok(visited.len() as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        let (_, visited) = self.analyse(None);
        let mut loop_obstacles = HashSet::new();
        for (i, (position, ..)) in visited.iter().enumerate() {
            debug!(i, ?position, "test");
            if self.analyse(Some(*position)).0 {
                loop_obstacles.insert(position);
            }
        }
        Ok(loop_obstacles.len() as ResultType)
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

    fn analyse(
        &self,
        additional_obstacle: Option<Point<isize>>,
    ) -> (bool, HashMap<Point<isize>, HashSet<Direction>>) {
        // Implement for problem
        let mut steps = 0;
        let mut guard_pos = self.find_guard();
        let mut visited: HashMap<Point<isize>, HashSet<Direction>> = HashMap::new();
        if matches!(additional_obstacle, Some(p) if p == guard_pos) {
            return (false, visited);
        }
        let mut direction = Direction::N;
        loop {
            if !visited.entry(guard_pos).or_default().insert(direction) {
                break (true, visited);
            }
            debug!(steps, ?guard_pos, ?direction, "stage");
            let front_pos = match direction {
                Direction::N => guard_pos.north(),
                Direction::E => guard_pos.east(),
                Direction::S => guard_pos.south(),
                Direction::W => guard_pos.west(),
                _ => panic!("unexpected direction {:?}", direction),
            };
            match match self.grid.get(&front_pos) {
                _ if additional_obstacle.map(|p| front_pos == p).unwrap_or(false) => Decision::Turn,
                Some('.') => Decision::Step,
                Some('#') => Decision::Turn,
                // Guard can't stand in front of themselves
                Some('^') => Decision::Step,
                Some(c) => panic!("Unknown entry in grid: {}", c),
                None => break (false, visited),
            } {
                Decision::Step => {
                    steps += 1;
                    guard_pos = front_pos;
                }
                Decision::Turn => {
                    direction = match direction {
                        Direction::N => Direction::E,
                        Direction::E => Direction::S,
                        Direction::S => Direction::W,
                        Direction::W => Direction::N,
                        _ => panic!("unexpected direction {:?}", direction),
                    }
                }
            }
        }
    }
}
