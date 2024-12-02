use std::{cmp::Ordering, io::{BufRead, BufReader}};
use itertools::Itertools;
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    reports: Vec<Vec<ResultType>>,
}
impl Solution {
    fn add_report(&mut self, levels: Vec<ResultType>) {
        self.reports.push(levels);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let levels = line.split_whitespace().flat_map(|v| v.parse()).collect();
            solution.add_report(levels);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        let answer = self.reports.iter().filter(|v| Self::is_safe_part1(*v)).count();
        Ok(answer as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        let answer = self.reports.iter().filter(|v| {
            if Self::is_safe_part1(*v) {
                true
            } else {
                for skip in 0..v.len() {
                    if Self::is_safe_part2(*v, skip) {
                        return true;
                    }
                }
                false
            }
        }).count();
        Ok(answer as ResultType)
    }
}
impl Solution {
    fn is_safe_part1(report: &Vec<ResultType>) -> bool {
        let mut dir = Ordering::Equal;
        let mut last = 0;
        for (i, cur) in report.iter().enumerate() {
            if i==0 {
                last = *cur;
                continue;
            }
            if last == *cur {
                return false;
            }
            if last > *cur {
                if dir == Ordering::Less {
                    return false;
                }
                if last - *cur > 3 {
                    return false;
                }
                dir = Ordering::Greater;
            }
            if last < *cur {
                if dir == Ordering::Greater {
                    return false;
                }
                if *cur - last > 3 {
                    return false;
                }
                dir = Ordering::Less;
            }
            last = *cur;
        }
        info!(report = debug(report), "safe");
        true
    }

    fn is_safe_part2(report: &Vec<ResultType>, skip: usize) -> bool {
        let mut dir = Ordering::Equal;
        let mut last = None;
        for (i, cur) in report.iter().enumerate() {
            if i == skip {
                continue;
            }
            if last == None {
                last = Some(*cur);
                continue;
            }
            if last.unwrap() == *cur {
                return false;
            }
            if last.unwrap() > *cur {
                if dir == Ordering::Less {
                    return false;
                }
                if last.unwrap() - *cur > 3 {
                    return false;
                }
                dir = Ordering::Greater;
            }
            if last.unwrap() < *cur {
                if dir == Ordering::Greater {
                    return false;
                }
                if *cur - last.unwrap() > 3 {
                    return false;
                }
                dir = Ordering::Less;
            }
            last = Some(*cur);
        }
        info!(report = debug(report), "safe");
        true
    }
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
