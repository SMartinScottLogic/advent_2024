use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::Point;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    grid: utils::SparseGrid<ResultType, isize>,
    part1_answer: ResultType,
    part2_answer: ResultType,
}
impl Solution {
    fn add_height(&mut self, x: usize, y: usize, c: char) {
        self.grid.set(
            &Point::new(x as isize, y as isize),
            c.to_digit(10).unwrap_or(10).into(),
        );
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
            for (x, c) in line.chars().enumerate() {
                solution.add_height(x, y, c);
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {
        let mut part1 = 0;
        let mut part2 = 0;
        for (p, c) in self.grid.iter() {
            if *c == 0 {
                let (reached, count) = self.get_reachable(p, *c);
                part1 += reached.len() as ResultType;
                part2 += count;
            }
        }
        self.part1_answer = part1;
        self.part2_answer = part2;
    }

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        Ok(self.part1_answer)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        Ok(self.part2_answer)
    }
}

impl Solution {
    fn get_reachable(
        &self,
        cur_position: &Point<isize>,
        c: ResultType,
    ) -> (HashSet<Point<isize>>, ResultType) {
        let mut reachable = HashSet::new();
        let mut total = 0;
        if c == 9 {
            reachable.insert(cur_position.to_owned());
            return (reachable, 1);
        }
        for next_pos in cur_position.cardinal() {
            match self.grid.get(&next_pos) {
                Some(nc) if *nc == c + 1 => {
                    let (a, b) = self.get_reachable(&next_pos, *nc);
                    for r in a {
                        reachable.insert(r);
                    }
                    total += b;
                }
                _ => {}
            }
        }
        (reachable, total)
    }
}
