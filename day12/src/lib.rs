use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::{grid::SparseGrid, point::Point};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    grid: SparseGrid<char, isize>,
    answer_part1: ResultType,
    answer_part2: ResultType,
}
impl Solution {
    fn set_plot(&mut self, x: usize, y: usize, c: char) {
        self.grid.set(&Point::new(x as isize, y as isize), c);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
            for (x, c) in line.chars().enumerate() {
                solution.set_plot(x, y, c);
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {
        // Implement for problem
        let mut regions = Vec::new();
        let mut have_seen = HashSet::new();
        for (cur_point, plot_type) in self.grid.iter() {
            if have_seen.contains(cur_point) {
                continue;
            }
            let region = self.grid.region_with_same_value(cur_point).unwrap();
            for n in region.points() {
                have_seen.insert(*n);
            }
            info!(?region, ?plot_type);
            regions.push((region, plot_type));
        }
        info!(?regions);
        self.answer_part1 = 0;
        for (region, plot_type) in regions {
            let area = region.area() as ResultType;
            let perimeter = region.perimeter() as ResultType;
            let num_sides = region.num_sides() as ResultType;
            info!(?plot_type, ?area, ?perimeter, ?num_sides);
            self.answer_part1 += area * perimeter;
            self.answer_part2 += area * num_sides;
        }
    }

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        Ok(self.answer_part1)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok(self.answer_part2)
    }
}
