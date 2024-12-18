use pathfinding::prelude::astar;
use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::{grid::Matrix, point::Point};

pub type ResultType = String;

#[derive(Debug, Default)]
pub struct Solution {
    bytes: Vec<(isize, isize)>,
}
impl Solution {
    fn add_byte(&mut self, x: isize, y: isize) {
        self.bytes.push((x, y));
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let (x, y) = line.split_once(",").unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            solution.add_byte(x, y);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, is_full: bool) -> Self::Result {
        // Implement for problem
        let startx = 0;
        let starty = 0;
        let (targetx, targety, num_drops): (isize, isize, usize) =
            if !is_full { (6, 6, 12) } else { (70, 70, 1024) };
        let mut matrix = Matrix::new();
        for (x, y) in self.bytes.iter().take(num_drops) {
            matrix.set(*x, *y, '#');
        }
        matrix.display_with_mapping(|c| {
            match c {
                '#' => "#",
                _ => ".",
            }
            .to_string()
        });
        if matrix.get(targetx, targety).is_none() {
            matrix.set(targetx, targety, '.');
        }
        let successors = |(px, py)| {
            Point::new(px, py)
                .cardinal()
                .iter()
                .filter(|p| {
                    p.x() >= 0
                        && p.y() >= 0
                        && p.x() <= matrix.max_x()
                        && p.y() <= matrix.max_y()
                        && matrix.get(p.x(), p.y()).unwrap_or(&'.') != &'#'
                })
                .map(|p| ((p.x(), p.y()), 1))
                .collect::<Vec<_>>()
        };
        let heuristic = |(px, py)| targetx.abs_diff(px) + targety.abs_diff(py);
        let success = |(px, py)| px == targetx && py == targety;
        let r = astar(
            &(startx, starty),
            |p| successors(*p),
            |p| heuristic(*p),
            |p| success(*p),
        );
        info!(?r);
        let r = r.map(|(_path, cost)| cost).unwrap_or(0);
        Ok(format!("{}", r))
    }

    fn answer_part2(&self, is_full: bool) -> Self::Result {
        // Implement for problem
        let startx = 0;
        let starty = 0;
        let (targetx, targety, mut num_drops): (isize, isize, usize) =
            if !is_full { (6, 6, 12) } else { (70, 70, 1024) };
        loop {
            let mut matrix = Matrix::new();
            for (x, y) in self.bytes.iter().take(num_drops) {
                matrix.set(*x, *y, '#');
            }
            /*
            matrix.display_with_mapping(|c| match c {
                '#' => "#",
                _ => ".",
            }.to_string());
            */
            if matrix.get(targetx, targety).is_none() {
                matrix.set(targetx, targety, '.');
            }
            let successors = |(px, py)| {
                Point::new(px, py)
                    .cardinal()
                    .iter()
                    .filter(|p| {
                        p.x() >= 0
                            && p.y() >= 0
                            && p.x() <= matrix.max_x()
                            && p.y() <= matrix.max_y()
                            && matrix.get(p.x(), p.y()).unwrap_or(&'.') != &'#'
                    })
                    .map(|p| ((p.x(), p.y()), 1))
                    .collect::<Vec<_>>()
            };
            let heuristic = |(px, py)| targetx.abs_diff(px) + targety.abs_diff(py);
            let success = |(px, py)| px == targetx && py == targety;
            let r = astar(
                &(startx, starty),
                |p| successors(*p),
                |p| heuristic(*p),
                |p| success(*p),
            );
            debug!(?r, num_drops);
            if r.is_none() {
                break;
            }
            num_drops += 1;
        }
        Ok(format!(
            "{},{}",
            self.bytes[num_drops - 1].0,
            self.bytes[num_drops - 1].1
        ))
    }
}
