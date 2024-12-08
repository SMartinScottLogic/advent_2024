use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::{Point, SparseGrid};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    grid: SparseGrid<char, isize>,
}
impl Solution {
    fn add_antenna(&mut self, x: usize, y: usize, c: char) {
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
                solution.add_antenna(x, y, c);
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut antinodes = HashSet::new();
        for (p1, c1) in self.grid.iter() {
            for (p2, c2) in self.grid.iter() {
                if p1 == p2 || c1 != c2 {
                    continue;
                }
                if *c1 == '.' {
                    continue;
                }
                debug!(?p1, ?p2, ?c1, ?c2);

                let dx = p1.x() - p2.x();
                let dy = p1.y() - p2.y();
                // antinode 1
                {
                    let x = p1.x() + dx;
                    let y = p1.y() + dy;
                    let p = Point::new(x, y);
                    if p != *p1 && p != *p2 && self.grid.contains(&p) {
                        debug!(?p, ?p1, ?p2, contained = self.grid.contains(&p));
                        antinodes.insert(p);
                    }
                }
                // antinode 2
                {
                    let x = p2.x() + dx;
                    let y = p2.y() + dy;
                    let p = Point::new(x, y);
                    if p != *p1 && p != *p2 && self.grid.contains(&p) {
                        debug!(?p, ?p1, ?p2, contained = self.grid.contains(&p));
                        antinodes.insert(p);
                    }
                }
                // antinode 3
                {
                    let x = p1.x() - dx;
                    let y = p1.y() - dy;
                    let p = Point::new(x, y);
                    if p != *p1 && p != *p2 && self.grid.contains(&p) {
                        debug!(?p, ?p1, ?p2, contained = self.grid.contains(&p));
                        antinodes.insert(p);
                    }
                }
                // antinode 4
                {
                    let x = p2.x() - dx;
                    let y = p2.y() - dy;
                    let p = Point::new(x, y);
                    if p != *p1 && p != *p2 && self.grid.contains(&p) {
                        debug!(?p, ?p1, ?p2, contained = self.grid.contains(&p));
                        antinodes.insert(p);
                    }
                }
            }
        }
        Ok(antinodes.len() as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut antinodes = HashSet::new();
        for (p1, c1) in self.grid.iter() {
            for (p2, c2) in self.grid.iter() {
                if p1 == p2 || c1 != c2 {
                    continue;
                }
                if *c1 == '.' {
                    continue;
                }
                debug!(?p1, ?p2, ?c1, ?c2);

                let dx = p1.x() - p2.x();
                let dy = p1.y() - p2.y();
                let mut x = p1.x();
                let mut y = p1.y();

                loop {
                    let p = Point::new(x, y);
                    if !self.grid.contains(&p) {
                        break;
                    }
                    antinodes.insert(p);
                    x -= dx;
                    y -= dy;
                }

                let mut x = p1.x();
                let mut y = p1.y();

                loop {
                    let p = Point::new(x, y);
                    if !self.grid.contains(&p) {
                        break;
                    }
                    antinodes.insert(p);
                    x += dx;
                    y += dy;
                }
            }
        }
        // Implement for problem
        Ok(antinodes.len() as ResultType)
    }
}
