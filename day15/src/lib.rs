use std::{collections::{HashSet, VecDeque}, io::{BufRead, BufReader}};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, Level};
use utils::{
    grid::{FixedGrid, Picture},
    point::{Direction, Point},
};

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
                    _ => todo!(),
                }
            }
        }

        for c in self.moves.chars() {
            let (direction, desired) = match c {
                'v' => (Direction::S, robot_pos.south()),
                '^' => (Direction::N, robot_pos.north()),
                '<' => (Direction::W, robot_pos.west()),
                '>' => (Direction::E, robot_pos.east()),
                _ => todo!(),
            };
            match map.get(desired.x() as isize, desired.y() as isize) {
                Some(Wall) => {}
                Some(Box) => {
                    if move_boxes_part1(&mut map, &desired, direction) {
                        robot_pos = desired;
                    }
                }
                Some(Empty) => {
                    robot_pos = desired;
                }
                v => todo!("Unexpected value {:?} at {:?}", v, robot_pos),
            }
        }
        let mut result = 0;
        debug!(?map);
        for py in 0..map.max_y() {
            for px in 0..map.max_x() {
                if map.get(px as isize, py as isize).unwrap() == &Box {result += 100 * py as ResultType + px as ResultType;}
            }
        }
        // for ((px, py), v) in map.iter() {
        //     if v == Box {
        //         info!(?px, ?py);
        //         // TODO Why do I need to add 1 here?
        //         result += 100 * py as ResultType + (px + 1) as ResultType;
        //     }
        // }
        display_map(&map);
        Ok(result)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        use CellType::*;
        let max_x = self.maprows[0].len();
        let max_y = self.maprows.len();
        let mut robot_pos = Point::new(0, 0);
        let mut map = FixedGrid::new(max_x * 2, max_y);
        for (y, row) in self.maprows.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                match c {
                    '@' => {
                        robot_pos = Point::new(x * 2, y);
                        map.set((x as isize) * 2, y as isize, Empty);
                        map.set((x as isize) * 2 + 1, y as isize, Empty);
                    }
                    '#' => {
                        map.set((x as isize) * 2, y as isize, Wall);
                        map.set((x as isize) * 2 + 1, y as isize, Wall);
                    }
                    'O' => {
                        map.set((x as isize) * 2, y as isize, BoxL);
                        map.set((x as isize) * 2 + 1, y as isize, BoxR);
                    }
                    '.' => {
                        map.set((x as isize) * 2, y as isize, Empty);
                        map.set((x as isize) * 2 + 1, y as isize, Empty);
                    }
                    _ => todo!(),
                }
            }
        }
        for c in self.moves.chars() {
            debug!(?robot_pos, ?c);

            let (direction, desired) = match c {
                'v' => (Direction::S, robot_pos.south()),
                '^' => (Direction::N, robot_pos.north()),
                '<' => (Direction::W, robot_pos.west()),
                '>' => (Direction::E, robot_pos.east()),
                _ => todo!(),
            };
            match map.get(desired.x() as isize, desired.y() as isize) {
                Some(Wall) => {}
                Some(Box) | Some(BoxL) | Some(BoxR) => {
                    if move_boxes_part2(&mut map, &desired, direction) {
                        robot_pos = desired;
                    }
                }
                Some(Empty) => {
                    robot_pos = desired;
                }
                v => todo!("Unexpected value {:?} at {:?}", v, robot_pos),
            }
            //display_map(&map);
            validate_map(&map);
        }
        let mut result = 0;
        debug!(?map);
        for py in 0..map.max_y() {
            for px in 0..map.max_x() {
                match map.get(px as isize, py as isize).unwrap() {
                    Empty => {},
                    Wall => {},
                    Box => {},
                    BoxL => {result += 100 * py as ResultType + px as ResultType;}
                    BoxR => {},
                }
            }
        }

        display_map(&map);
        debug!(?robot_pos);

        Ok(result)
    }
}

fn display_map(map: &FixedGrid<CellType>) {
    use CellType::*;
    Picture::from(map.clone()).display_with_mapping(|v| match v {
        Empty => ".",
        Wall => "#",
        Box => "O",
        BoxL => "[",
        BoxR => "]",
    });
}
fn validate_map(map: &FixedGrid<CellType>
) {
    for py in 0..map.max_y() {
        for px in 0..map.max_x() {
            let c = map.get(px as isize, py as isize).unwrap();
        match c {
            CellType::BoxL => {
                match map.get((px+1) as isize, py as isize).unwrap() {
                    CellType::BoxR => {}
                    c => panic!("expected ']' at {},{} saw {:?}", px+1, py, c)
                }
            }
            CellType::BoxR => {
                match map.get((px-1) as isize, py as isize).unwrap() {
                    CellType::BoxL => {}
                    c => panic!("expected ']' at {},{} saw {:?}", px-1, py, c)
                }
            }
            _ => {}
        }
    }}
}
fn move_boxes_part1(
    map: &mut FixedGrid<CellType>,
    desired: &Point<usize>,
    direction: Direction,
) -> bool {
    use CellType::*;
    let mut pos = desired.to_owned();
    let can_move = loop {
        debug!(?pos);
        match map.get(pos.x() as isize, pos.y() as isize) {
            Some(Empty) => {
                break true;
            }
            Some(Wall) => {
                break false;
            }
            Some(Box) => {}
            Some(BoxL) => {}
            Some(BoxR) => {}
            c => todo!("Unexpected: {:?}", c),
        }
        // MUST be a box to get here
        pos = match direction {
            Direction::N => pos.north(),
            Direction::E => pos.east(),
            Direction::S => pos.south(),
            Direction::W => pos.west(),
            _ => todo!(),
        };
    };
    if can_move {
        loop {
            let next_pos = match direction {
                Direction::N => pos.south(),
                Direction::E => pos.west(),
                Direction::S => pos.north(),
                Direction::W => pos.east(),
                _ => todo!(),
            };
            debug!(?pos, ?next_pos);
            let cur = *map.get(pos.x() as isize, pos.y() as isize).unwrap();
            let next = *map
                .get(next_pos.x() as isize, next_pos.y() as isize)
                .unwrap();
            map.set(pos.x() as isize, pos.y() as isize, next);
            map.set(next_pos.x() as isize, next_pos.y() as isize, cur);
            debug!(?pos, ?next_pos, ?cur, ?next, ?desired, "swap");
            pos = next_pos;
            if pos == *desired {
                break;
            }
        }
    }
    can_move
}

fn move_boxes_part2(
    map: &mut FixedGrid<CellType>,
    desired: &Point<usize>,
    direction: Direction,
) -> bool {
    use CellType::*;
    // East and West same as part 1
    if direction == Direction::E || direction == Direction::W {
        move_boxes_part1(map, desired, direction)
    } else {
        let mut probe = match map.get(desired.x() as isize, desired.y() as isize) {
            Some(BoxL) => vec![desired.to_owned(), desired.east()],
            Some(BoxR) => vec![desired.to_owned(), desired.west()],
            c => panic!("Unexpected {:?}", c)
        };
        let mut to_move: VecDeque<HashSet<Point<usize>>> = VecDeque::new();


        loop {
            let mut next_probe = Vec::new();
            let mut n = HashSet::new();
            for p in &probe {
                match map.get(p.x() as isize, p.y() as isize) {
                    Some(Empty) => {}
                    Some(Wall) => {
                        return false;
                    }
                    Some(Box) => todo!(),
                    Some(BoxL) => {
                        n.insert(*p);
                        n.insert(p.east());
                    }
                    Some(BoxR) => {
                        n.insert(*p);
                        n.insert(p.west());
                    }
                    None => panic!(),
                }
            }
            debug!(?n);
            for np in &n {
                let np = match direction {
                    Direction::N => np.north(),
                    Direction::S => np.south(),
                    _ => panic!(),
                };
                next_probe.push(np);
            }
            debug!(?probe, ?next_probe);
            if next_probe.is_empty() {
                break;
            }
            to_move.push_front(n);
            probe = next_probe;
        }
        for m in &to_move {
            debug!(?m);
        }
        for row in to_move {
            for pos in row {
                let next_pos = match direction {
                    Direction::N => pos.north(),
                    Direction::S => pos.south(),
                    _ => todo!(),
                };
                debug!(?pos, ?next_pos);
                let cur = *map.get(pos.x() as isize, pos.y() as isize).unwrap();
                let next = *map
                    .get(next_pos.x() as isize, next_pos.y() as isize)
                    .unwrap();
                map.set(pos.x() as isize, pos.y() as isize, next);
                map.set(next_pos.x() as isize, next_pos.y() as isize, cur);
                debug!(?pos, ?next_pos, ?cur, ?next, ?desired, "swap");
            }
        }
        true
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum CellType {
    #[default]
    Empty,
    Wall,
    Box,
    BoxL,
    BoxR,
}
