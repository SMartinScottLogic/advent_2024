use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::{grid::{FixedGrid, Picture}, point::{Direction, Point}};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    maprows: Vec<String>,
    moves: String,
}
impl Solution {
    fn add_maprow(&mut self, row: String) {
        self.maprows.push(row);
    }
    fn add_moves(&mut self, line: String) {
        self.moves.push_str(&line);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        let mut stage = 0;
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            if line.is_empty() {
                stage = 1;
                continue;
            }
            if stage == 0 {
                solution.add_maprow(line);
            } else if stage == 1 {
                solution.add_moves(line);
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        use CellType::*;
        // Implement for problem
        let max_x = self.maprows[0].len();
        let max_y = self.maprows.len();
        let mut robot_pos = Point::new(0, 0);
        let mut map = FixedGrid::new(max_x, max_y);
        for (y, row) in self.maprows.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                match c {
                    '@' => {
                        robot_pos = Point::new(x, y);
                        map.set(x as isize, y as isize, Empty);
                    }
                    '#' => {
                        map.set(x as isize, y as isize, Wall);
                    }
                    'O' => {
                        map.set(x as isize, y as isize, Box);
                    }
                    '.' => {
                        map.set(x as isize, y as isize, Empty);
                    }
                    _ => todo!()
                }
            }
        }
    
        for c in self.moves.chars() {
            let (direction, desired) = match c {
                'v' => (Direction::S, robot_pos.south()),
                '^' => (Direction::N,robot_pos.north()),
                '<' => (Direction::W,robot_pos.west()),
                '>' => (Direction::E,robot_pos.east()),
                _ => todo!()
            };
            match map.get(desired.x() as isize, desired.y() as isize) {
                Some(Wall) => {}
                Some(Box) => {
                    if move_boxes(&mut map, &desired, direction) {
                        robot_pos = desired;
                    }
                }
                Some(Empty) => {robot_pos = desired;}
                v => todo!("Unexpected value {:?} at {:?}", v, robot_pos)
            }

        }
        let mut result = 0;
        info!(?map);
        for ((px, py), v) in map.iter() {
            if v == Box {
                info!(?px, ?py);
                // TODO Why do I need to add 1 here?
                result += 100 * py as ResultType + (px+1) as ResultType;
            }
        }
        Picture::from(map).display_with_mapping(|v| match v {
            Empty => ".",
            Wall => "#",
            Box => "O",
        });
        Ok(result)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok(0)
    }
}

fn move_boxes(map: &mut FixedGrid<CellType>, desired: &Point<usize>, direction: Direction) -> bool {
    use CellType::*;
    let mut pos = desired.to_owned();
    let can_move = loop {
        info!(?pos);
        match map.get(pos.x() as isize, pos.y() as isize) {
            Some(Empty) => {break true;}
            Some(Wall) => {break false;}
            Some(Box) => {}
            None => todo!()
        }
        // MUST be a box to get here
        pos = match direction {
            Direction::N => pos.north(),
            Direction::E => pos.east(),
            Direction::S => pos.south(),
            Direction::W => pos.west(),
            _ => todo!()
        };
    };
    if can_move {
        loop {
            let next_pos = match direction {
                Direction::N => pos.south(),
                Direction::E => pos.west(),
                Direction::S => pos.north(),
                Direction::W => pos.east(),
                _ => todo!()
            };
            info!(?pos, ?next_pos);
            let cur = *map.get(pos.x() as isize, pos.y() as isize).unwrap();
            let next = *map.get(next_pos.x() as isize, next_pos.y() as isize).unwrap();
            map.set(pos.x() as isize, pos.y() as isize, next);
            map.set(next_pos.x() as isize, next_pos.y() as isize, cur);
            pos = next_pos;
            if pos == *desired {
                break;
            }
        }
    }
    can_move
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum CellType {
    #[default]
    Empty,
    Wall,
    Box,
}
#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    use tracing_test::traced_test;
    use utils::Solution;

    #[test]
    #[traced_test]
    fn read() {
        let input = "replace for problem";
        let r = BufReader::new(input.as_bytes());
        let s = crate::Solution::try_from(r).unwrap();
        assert_eq!(0 as ResultType, s.answer_part1(false).unwrap());
    }
}
