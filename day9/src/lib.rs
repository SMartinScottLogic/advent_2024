use std::io::{BufRead, BufReader};
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
            blocks[s] = blocks[e].clone();
            blocks[e] = Block::Empty;
        }
        debug!(?blocks, "blocks");

        Ok(checksum(&blocks))
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        let mut blocks = diskmap(&self.diskmap);

        loop {
            let mut s = 0;
            let mut e = blocks.len() - 1;

            while blocks[e] == Block::Empty {
                e -= 1;
            }

            blocks[s] = blocks[e].clone();
            blocks[e] = Block::Empty;
            if s == 0 {
                break;
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
            blocks.push(s.clone());
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Block {
    Empty,
    FileBlock(ResultType),
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
