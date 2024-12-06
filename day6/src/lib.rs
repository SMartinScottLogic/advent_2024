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
    guard_pos: Point<isize>,
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
    fn analyse(&mut self, _is_full: bool) {
        self.guard_pos = self
            .grid
            .iter()
            .filter(|(_point, c)| *c == &'^')
            .map(|(point, _c)| point)
            .cloned()
            .next()
            .unwrap();
    }

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        let (looped, visited, ..) = self.analyse(self.guard_pos, Direction::N, None);
        assert!(!looped);
        Ok(visited.len() as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        let (_, visited, first_visited) = self.analyse(self.guard_pos, Direction::N, None);
        debug!(?first_visited, "first_visits");
        let mut loop_obstacles = HashSet::new();
        for (i, (position, ..)) in visited.iter().enumerate() {
            debug!(i, ?position, "test");
            let direction = first_visited.get(position).unwrap();
            let guard_pos = match direction {
                Direction::N => position.south(),
                Direction::E => position.west(),
                Direction::S => position.north(),
                Direction::W => position.east(),
                _ => panic!()
            };
            if self.analyse(guard_pos, *direction, Some(*position)).0 {
                loop_obstacles.insert(position);
            }
        }
        Ok(loop_obstacles.len() as ResultType)
    }
}

impl Solution {
    fn analyse(
        &self,
        mut guard_pos: Point<isize>,
        mut direction: Direction,
        additional_obstacle: Option<Point<isize>>,
    ) -> (bool, HashMap<Point<isize>, HashSet<Direction>>, HashMap<Point<isize>, Direction>) {
        // Implement for problem
        let mut steps = 0;
        //let mut guard_pos = self.guard_pos;
        let mut visited: HashMap<Point<isize>, HashSet<Direction>> = HashMap::new();
        let mut first_visited = HashMap::new();
        if matches!(additional_obstacle, Some(p) if p == guard_pos) {
            return (false, visited, first_visited);
        }
        //let mut direction = Direction::N;
        loop {
            if !visited.entry(guard_pos).or_default().insert(direction) {
                break (true, visited, first_visited);
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
                None => break (false, visited, first_visited),
            } {
                Decision::Step => {
                    steps += 1;
                    first_visited.entry(front_pos).or_insert(direction);
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
