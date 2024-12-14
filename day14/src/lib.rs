use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};
use tracing::enabled;
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::Picture;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    robots: Vec<Robot>,
}
impl Solution {
    fn add_robot(&mut self, robot: Robot) {
        self.robots.push(robot);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let robot = line.into();
            solution.add_robot(robot);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let num_steps = 100;
        let max_x = 101;
        let max_y = 103;
        let mut grid: HashMap<(isize, isize), usize> = HashMap::new();
        for robot in &self.robots {
            let x = (robot.px + robot.vx * num_steps).rem_euclid(max_x);
            let y = (robot.py + robot.vy * num_steps).rem_euclid(max_y);
            *grid.entry((x, y)).or_default() += 1;
        }
        debug!(?grid);
        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;
        for ((px, py), c) in grid {
            if px < max_x >> 1 && py < max_y >> 1 {
                q1 += c;
            } else if px > max_x >> 1 && py < max_y >> 1 {
                q2 += c;
            } else if px < max_x >> 1 && py > max_y >> 1 {
                q3 += c;
            } else if px > max_x >> 1 && py > max_y >> 1 {
                q4 += c;
            }
        }
        info!(q1, q2, q3, q4);
        let r = q1 * q2 * q3 * q4;
        // Implement for problem
        Ok(r as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut num_steps = 0;
        let max_x = 101;
        let max_y = 103;
        loop {
            let mut grid = Picture::new(max_x, max_y);
            //let mut grid = utils::Matrix::new();
            num_steps += 1;
            for robot in &self.robots {
                let x = (robot.px + robot.vx * num_steps).rem_euclid(max_x as isize);
                let y = (robot.py + robot.vy * num_steps).rem_euclid(max_y as isize);
                let c = grid.get(x, y).unwrap_or(&0).to_owned();
                grid.set(x, y, c + 1);
            }
            if grid.iter().all(|(_, v)| v <= 1) {
                if enabled!(Level::INFO) {
                    grid.display_with_mapping(|v| if v == 0 { ' ' } else { '#' }.to_string());
                    let clumpiness = grid.clumpiness();
                    info!(num_steps, clumpiness);
                }
                if num_steps > 10 {
                    break;
                }
            }
        }

        // Not: 7604, but 7603 ?!?
        Ok(num_steps as ResultType)
    }
}

#[derive(Debug, Clone)]
struct Robot {
    px: isize,
    py: isize,
    vx: isize,
    vy: isize,
}
impl From<String> for Robot {
    fn from(value: String) -> Self {
        let r = regex::Regex::new(
            r"^p=(?<x>[-+]?\d+),(?<y>[-+]?\d+) v=(?<vx>[-+]?\d+),(?<vy>[-+]?\d+)$",
        )
        .unwrap();
        let c = r.captures(&value).unwrap();
        let px = c.name("x").unwrap().as_str().parse().unwrap();
        let py = c.name("y").unwrap().as_str().parse().unwrap();
        let vx = c.name("vx").unwrap().as_str().parse().unwrap();
        let vy = c.name("vy").unwrap().as_str().parse().unwrap();
        Self { px, py, vx, vy }
    }
}
