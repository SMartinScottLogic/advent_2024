use array2d::Array2D;
use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::{Direction, Point};

pub type ResultType = u64;

#[derive(Debug)]
enum Decision {
    Step,
    Turn,
}
#[derive(Debug)]
pub struct Solution {
    grid: array2d::Array2D<char>,
    guard_pos: Point<isize>,
}
impl Solution {}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut rows = Vec::new();
        for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
            rows.push(line.chars().collect());
        }
        let solution = Solution {
            grid: Array2D::from_columns(&rows).unwrap(),
            guard_pos: Point::default(),
        };
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {
        self.guard_pos = self
            .grid
            .enumerate_row_major()
            .filter(|(_pos, c)| *c == &'^')
            .map(|((x, y), _c)| Point::new(x as isize, y as isize))
            .next()
            .unwrap();
    }

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        let AnalyseResult(looped, visited, ..) = self.analyse(self.guard_pos, Direction::N, None);
        assert!(!looped);
        //panic!();
        Ok(visited
            .elements_row_major_iter()
            .filter(|v| v[0].is_some())
            .count() as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        let mut count = 0;
        let AnalyseResult(_, visited, first_visited) =
            self.analyse(self.guard_pos, Direction::N, None);
        debug!(?first_visited, "first_visits");
        for (i, (position, ..)) in visited.enumerate_row_major().enumerate() {
            let position = Point::new(position.0 as isize, position.1 as isize);
            debug!(i, ?position, "test");
            if let Some(direction) = Self::get(&first_visited, position).unwrap() {
                let guard_pos = match direction {
                    Direction::N => position.south(),
                    Direction::E => position.west(),
                    Direction::S => position.north(),
                    Direction::W => position.east(),
                    _ => panic!(),
                };
                if self.analyse(guard_pos, *direction, Some(position)).0 {
                    count += 1;
                }
            }
        }
        Ok(count)
    }
}

struct AnalyseResult(
    bool,
    Array2D<[Option<Direction>; 4]>,
    Array2D<Option<Direction>>,
);

impl Solution {
    fn analyse(
        &self,
        mut guard_pos: Point<isize>,
        mut direction: Direction,
        additional_obstacle: Option<Point<isize>>,
    ) -> AnalyseResult {
        // Implement for problem
        let mut steps = 0;
        let mut visited =
            Array2D::filled_with([None; 4], self.grid.num_rows(), self.grid.num_columns());
        let mut first_visited =
            Array2D::filled_with(None, self.grid.num_rows(), self.grid.num_columns());
        if matches!(additional_obstacle, Some(p) if p == guard_pos) {
            return AnalyseResult(false, visited, first_visited);
        }
        loop {
            if let Some(v) = Self::get_mut(&mut visited, guard_pos) {
                let mut found = false;
                let mut insert = None;
                for (i, v) in v.iter().enumerate() {
                    match v {
                        Some(d) if d == &direction => {
                            found = true;
                            break;
                        }
                        None if insert.is_none() => {
                            insert = Some(i);
                        }
                        _ => {}
                    }
                }
                if found {
                    return AnalyseResult(true, visited, first_visited);
                } else {
                    v[insert.unwrap()] = Some(direction);
                }
            }

            debug!(steps, ?guard_pos, ?direction, "stage");
            let front_pos = match direction {
                Direction::N => guard_pos.north(),
                Direction::E => guard_pos.east(),
                Direction::S => guard_pos.south(),
                Direction::W => guard_pos.west(),
                _ => panic!("unexpected direction {:?}", direction),
            };
            match match Self::get(&self.grid, front_pos) {
                _ if additional_obstacle.map(|p| front_pos == p).unwrap_or(false) => Decision::Turn,
                Some('.') => Decision::Step,
                Some('#') => Decision::Turn,
                // Guard can't stand in front of themselves
                Some('^') => Decision::Step,
                Some(c) => panic!("Unknown entry in grid: {}", c),
                None => break AnalyseResult(false, visited, first_visited),
            } {
                Decision::Step => {
                    steps += 1;
                    if let Some(e) = Self::get_mut(&mut first_visited, front_pos) {
                        if e.is_none() {
                            *e = Some(direction);
                        }
                    }
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

    fn get<T>(a: &Array2D<T>, pos: Point<isize>) -> Option<&T> {
        if pos.x() < 0 || pos.y() < 0 {
            None
        } else {
            a.get(pos.x() as usize, pos.y() as usize)
        }
    }

    fn get_mut<T>(a: &mut Array2D<T>, pos: Point<isize>) -> Option<&mut T> {
        if pos.x() < 0 || pos.y() < 0 {
            None
        } else {
            a.get_mut(pos.x() as usize, pos.y() as usize)
        }
    }
}
