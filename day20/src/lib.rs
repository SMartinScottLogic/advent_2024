use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::grid::Matrix;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    grid: Matrix<char>,
}
impl Solution {
    fn set_grid(&mut self, x: usize, y: usize, c: char) {
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
                solution.set_grid(x, y, c);
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        // Find start and end
        let mut start = (0, 0, false);
        let mut end = (0, 0);
        for y in 0..=self.grid.max_y() {
            for x in 0..=self.grid.max_x() {
                match self.grid.get(x, y) {
                    Some('S') => {
                        start = (x, y, false);
                    }
                    Some('E') => end = (x, y),
                    _ => {}
                }
            }
        }
        // Get distances along best path
        let d = self.distance(start.0, start.1, end.0, end.1);
        let max = d.get(&(end.0, end.1)).unwrap();
        debug!(max, ?d);

        let mut r = 0;
        for y in 0..=self.grid.max_y() {
            for x in 0..=self.grid.max_x() {
                if let Some(c) = self.grid.get(x, y) {
                    if *c != '.' && *c != 'S' && *c != 'E' {
                        continue;
                    }
                    if let Some(cost1) = d.get(&(x, y)) {
                        for (dx, dy) in [
                            (2, 0),
                            (1, 1),
                            (0, 2),
                            (-1, 1),
                            (-2, 0),
                            (-1, -1),
                            (0, -2),
                            (1, -1),
                        ] {
                            let nx = x + dx;
                            let ny = y + dy;
                            if let Some(c) = self.grid.get(nx, ny) {
                                if *c != '.' && *c != 'S' && *c != 'E' {
                                    continue;
                                }
                            }
                            if let Some(cost2) = d.get(&(nx, ny)) {
                                let saving = cost1 - cost2 - 2;
                                if saving >= 100 {
                                    r += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Find start and end
        let mut start = (0, 0, false);
        let mut end = (0, 0);
        for y in 0..=self.grid.max_y() {
            for x in 0..=self.grid.max_x() {
                match self.grid.get(x, y) {
                    Some('S') => {
                        start = (x, y, false);
                    }
                    Some('E') => end = (x, y),
                    _ => {}
                }
            }
        }
        // Get distances along best path
        let d = self.distance(start.0, start.1, end.0, end.1);
        let max = d.get(&(end.0, end.1)).unwrap();
        debug!(max, ?d);

        let mut r = 0;
        for y in 0..=self.grid.max_y() {
            for x in 0..=self.grid.max_x() {
                if let Some(c) = self.grid.get(x, y) {
                    if *c != '.' && *c != 'S' && *c != 'E' {
                        continue;
                    }
                    if let Some(cost1) = d.get(&(x, y)) {
                        for dy in -20..=20_isize {
                            for dx in -20..=20_isize {
                                let manhattan_distance = (dx.abs() + dy.abs()) as i32;
                                if manhattan_distance > 20 {
                                    continue;
                                }
                                let nx = x + dx;
                                let ny = y + dy;
                                if let Some(c) = self.grid.get(nx, ny) {
                                    if *c != '.' && *c != 'S' && *c != 'E' {
                                        continue;
                                    }
                                }
                                if let Some(cost2) = d.get(&(nx, ny)) {
                                    let saving = cost1 - cost2 - manhattan_distance;
                                    if saving >= 100 {
                                        r += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(r)
    }
}

impl Solution {
    fn distance(
        &self,
        x: isize,
        y: isize,
        end_x: isize,
        end_y: isize,
    ) -> HashMap<(isize, isize), i32> {
        let mut queue = VecDeque::new();
        queue.push_back((x, y, 0));
        let mut visited = HashSet::new();
        visited.insert((x, y));
        let mut distances = HashMap::new();
        distances.insert((x, y), 0);
        while let Some((px, py, d)) = queue.pop_front() {
            debug!(px, py, d, "pop");
            if px == end_x && py == end_y {
                debug!("found end");
                continue;
            }
            visited.insert((px, py));
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let nx = px + dx;
                let ny = py + dy;
                let reachable = matches!(self.grid.get(nx, ny), Some('.') | Some('S') | Some('E'));
                if reachable {
                    if distances.contains_key(&(nx, ny)) {
                        continue;
                    }
                    distances.insert((nx, ny), d + 1);
                    if !visited.contains(&(nx, ny)) {
                        debug!(nx, ny, ?distances, "enqueue");
                        queue.push_back((nx, ny, d + 1));
                    }
                }
            }
        }
        distances
    }
}
