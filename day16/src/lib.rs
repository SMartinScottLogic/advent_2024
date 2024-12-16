use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::{grid::Matrix, point::Direction};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    grid: Matrix<char>,
}
impl Solution {
    fn set_tile(&mut self, x: usize, y: usize, c: char) {
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
                solution.set_tile(x, y, c);
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut start = (Direction::E, -1, -1);
        let mut end = (-1, -1);
        for y in 0..=self.grid.max_y() {
            for x in 0..=self.grid.max_x() {
                match self.grid.get(x, y) {
                    Some('S') => {
                        start = (Direction::E, x, y);
                    }
                    Some('E') => {
                        end = (x, y);
                    }
                    _ => {}
                }
            }
        }

        let answer = if let Some((_route, cost)) = pathfinding::directed::astar::astar(
            &start,
            |p| Self::successors(&self.grid, p.0, p.1, p.2),
            |p| Self::heuristic(p.0, p.1, p.2, &end),
            |p| Self::success(p.0, p.1, p.2, &end),
        ) {
            cost
        } else {
            0
        };
        // Implement for problem
        Ok(answer)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut start = (Direction::E, -1, -1);
        let mut end = (-1, -1);
        for y in 0..=self.grid.max_y() {
            for x in 0..=self.grid.max_x() {
                match self.grid.get(x, y) {
                    Some('S') => {
                        start = (Direction::E, x, y);
                    }
                    Some('E') => {
                        end = (x, y);
                    }
                    _ => {}
                }
            }
        }

        let answer = if let Some((routes, cost)) = pathfinding::directed::astar::astar_bag_collect(
            &start,
            |p| Self::successors(&self.grid, p.0, p.1, p.2),
            |p| Self::heuristic(p.0, p.1, p.2, &end),
            |p| Self::success(p.0, p.1, p.2, &end),
        ) {
            debug!(?routes, cost);
            let nodes = routes
                .iter()
                .flat_map(|v| v.iter().map(|&(_facing, x, y)| (x, y)))
                .collect::<HashSet<_>>();
            nodes.len() as ResultType
        } else {
            0
        };
        // Implement for problem
        Ok(answer)
    }
}

impl Solution {
    fn success(_facing: Direction, x: isize, y: isize, end: &(isize, isize)) -> bool {
        end.0 == x && end.1 == y
    }
    fn heuristic(_facing: Direction, x: isize, y: isize, end: &(isize, isize)) -> ResultType {
        (end.0.abs_diff(x) + end.1.abs_diff(y)) as ResultType
    }
    fn successors(
        grid: &Matrix<char>,
        facing: Direction,
        x: isize,
        y: isize,
    ) -> Vec<((Direction, isize, isize), ResultType)> {
        match facing {
            Direction::N => vec![
                ((Direction::E, x, y), 1000),
                ((Direction::W, x, y), 1000),
                ((Direction::N, x, y - 1), 1),
            ],
            Direction::S => vec![
                ((Direction::E, x, y), 1000),
                ((Direction::W, x, y), 1000),
                ((Direction::S, x, y + 1), 1),
            ],
            Direction::E => vec![
                ((Direction::N, x, y), 1000),
                ((Direction::S, x, y), 1000),
                ((Direction::E, x + 1, y), 1),
            ],
            Direction::W => vec![
                ((Direction::N, x, y), 1000),
                ((Direction::S, x, y), 1000),
                ((Direction::W, x - 1, y), 1),
            ],
            _ => panic!(),
        }
        .into_iter()
        .filter(|&((_facing, x, y), _c)| *grid.get(x, y).unwrap() != '#')
        .collect::<Vec<_>>()
    }
}
