use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    diskmap: Vec<ResultType>,
}
impl Solution {
    fn set_diskmap(&mut self, map: Vec<ResultType>) {
        self.diskmap = map;
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            let diskmap = line
                .chars()
                .map(|c| format!("{}", c).parse().unwrap())
                .collect();
            // Implement for problem
            solution.set_diskmap(diskmap);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        let mut blocks = diskmap(&self.diskmap);
        let mut s = 0;
        let mut e = blocks.len() - 1;

        loop {
            while blocks[s] != Block::Empty {
                s += 1;
            }
            while blocks[e] == Block::Empty {
                e -= 1;
            }
            if s >= e {
                break;
            }
            blocks[s] = blocks[e];
            blocks[e] = Block::Empty;
        }
        debug!(?blocks, "blocks");

        Ok(checksum(&blocks))
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        let mut handled = HashSet::new();
        let mut blocks = diskmap(&self.diskmap);

        let mut e = (blocks.len() - 1) as isize;

        loop {
            while e >= 0
                && (blocks[e as usize] == Block::Empty || handled.contains(&blocks[e as usize]))
            {
                e -= 1;
                debug!(e, end = (e >= 0));
            }
            if e < 0 {
                break;
            }
            handled.insert(blocks[e as usize]);
            let mut num_e = 0;
            while (e - num_e) >= 0 && blocks[e as usize] == blocks[(e - num_e) as usize] {
                num_e += 1;
            }
            debug!(block = ?blocks[e as usize], num_e);
            let mut s = 0;
            let (s, num_s, can_move) = loop {
                while s < (e - num_e) as isize && blocks[s as usize] != Block::Empty {
                    s += 1;
                }
                if s >= (e - num_e) as isize || blocks[s as usize] != Block::Empty {
                    break (s, 0, false);
                }
                let mut num_s = 1;
                while num_s < num_e
                    && ((s + num_s) as usize) < blocks.len()
                    && blocks[(s + num_s) as usize] == Block::Empty
                {
                    num_s += 1;
                }
                if num_s >= num_e {
                    break (s, num_s, true);
                }
                s += num_s;
            };
            debug!(s, num_s, e, num_s, can_move);
            if can_move {
                for i in 0..num_e {
                    blocks[(s + i) as usize] = blocks[(e - num_e + 1 + i) as usize];
                    blocks[(e - num_e + 1 + i) as usize] = Block::Empty;
                }
                if event_enabled!(Level::DEBUG) {
                    debug!(
                        result = blocks.iter().fold(String::new(), |mut a, b| {
                            match b {
                                Block::Empty => a.push('.'),
                                Block::FileBlock(id) => a.push_str(&format!("{}", id)),
                            };
                            a
                        })
                    );
                }
            }
        }
        debug!(?blocks, "blocks");

        Ok(checksum(&blocks))
    }
}

fn diskmap(map: &[ResultType]) -> Vec<Block> {
    let mut blocks = Vec::new();
    for (id, c) in map.iter().enumerate() {
        let s = if id % 2 == 0 {
            Block::FileBlock((id / 2).try_into().unwrap())
        } else {
            Block::Empty
        };
        for _ in 0..*c {
            blocks.push(s);
        }
    }
    debug!(?blocks, "blocks");
    blocks
}

fn checksum(blocks: &[Block]) -> ResultType {
    blocks
        .iter()
        .enumerate()
        .map(|(pos, block)| {
            if let Block::FileBlock(id) = block {
                id * (pos as ResultType)
            } else {
                0
            }
        })
        .sum()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Block {
    Empty,
    FileBlock(ResultType),
}
