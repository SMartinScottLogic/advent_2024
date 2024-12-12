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
    grid: utils::SparseGrid<char, isize>,
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
            let region = grow_region(*cur_point, *plot_type, &self.grid);
            for n in &region {
                have_seen.insert(*n);
            }
            info!(?region, ?plot_type);
            regions.push((region, plot_type));
        }
        info!(?regions);
        self.answer_part1 = 0;
        for (region, plot_type) in regions {
            let area = calculate_area(&region);
            let perimeter = calculate_perimeter(&region);
            let num_sides = calculate_num_sides(&region);
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

fn grow_region(
    cur_point: Point<isize>,
    plot_type: char,
    grid: &utils::SparseGrid<char, isize>,
) -> HashSet<Point<isize>> {
    let mut region = HashSet::new();
    let mut remaining = Vec::new();
    remaining.push(cur_point);
    while let Some(cur) = remaining.pop() {
        if !region.insert(cur) {
            continue;
        }
        for neigh in cur.cardinal() {
            if let Some(v) = grid.get(&neigh) {
                if *v == plot_type {
                    remaining.push(neigh);
                }
            }
        }
    }
    region
}

fn calculate_area(region: &HashSet<Point<isize>>) -> ResultType {
    region.len() as ResultType
}

fn calculate_perimeter(region: &HashSet<Point<isize>>) -> ResultType {
    let mut perimeter = 0;
    let mut shared = 0;
    for e in region {
        for n in e.cardinal() {
            if region.contains(&n) {
                shared += 1;
                debug!(?e, ?n, "shared");
            }
        }
        perimeter += 4;
    }
    debug!(?perimeter, ?shared, ?region);

    (perimeter - shared) as ResultType
}

fn calculate_num_sides(region: &HashSet<Point<isize>>) -> ResultType {
    let mut num_sides = 0;
    for e in region {
        // Outer corners
        if !region.contains(&e.west()) && !region.contains(&e.south()) {
            num_sides += 1;
        }
        if !region.contains(&e.east()) && !region.contains(&e.south()) {
            num_sides += 1;
        }
        if !region.contains(&e.west()) && !region.contains(&e.north()) {
            num_sides += 1;
        }
        if !region.contains(&e.east()) && !region.contains(&e.north()) {
            num_sides += 1;
        }
        // Inner corners
        if region.contains(&e.west())
            && region.contains(&e.south())
            && !region.contains(&e.southwest())
        {
            num_sides += 1;
        }
        if region.contains(&e.east())
            && region.contains(&e.south())
            && !region.contains(&e.southeast())
        {
            num_sides += 1;
        }
        if region.contains(&e.west())
            && region.contains(&e.north())
            && !region.contains(&e.northwest())
        {
            num_sides += 1;
        }
        if region.contains(&e.east())
            && region.contains(&e.north())
            && !region.contains(&e.northeast())
        {
            num_sides += 1;
        }
    }
    num_sides as ResultType
}
