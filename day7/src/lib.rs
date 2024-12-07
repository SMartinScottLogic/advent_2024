use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    equations: Vec<(ResultType, Vec<ResultType>)>,
}
impl Solution {
    fn add_equation(&mut self, result: ResultType, fields: Vec<ResultType>) {
        self.equations.push((result, fields));
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let (result, rhs) = line.split_once(":").unwrap();
            let result = result.parse().unwrap();
            let rhs = rhs.split_whitespace().map(|v| v.parse().unwrap()).collect();
            solution.add_equation(result, rhs);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        let mut total = 0;
        for (answer, values) in &self.equations {
            if can_be_true(answer, values, false) {
                total += answer;
            }
        }
        Ok(total)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        let mut total = 0;
        for (answer, values) in &self.equations {
            if can_be_true(answer, values, true) {
                total += answer;
            }
        }
        Ok(total)
    }
}

fn can_be_true(answer: &ResultType, values: &[ResultType], is_part2: bool) -> bool {
    let mut operators = values.iter().map(|_| '*').skip(1).collect::<Vec<_>>();
    test_all_up_to(
        operators.len() - 1,
        answer,
        values,
        &mut operators,
        is_part2,
    )
}

fn evaluate(values: &[ResultType], operators: &[char], is_part2: bool) -> ResultType {
    let mut answer = values[0];
    for p in 0..operators.len() {
        let operator = operators[p];
        let rhs = values[p + 1];
        answer = match operator {
            '*' => answer * rhs,
            '+' => answer + rhs,
            '|' if is_part2 => concatenate(answer, rhs),
            _ => panic!(),
        }
    }
    //info!(?answer);
    answer
}

fn test_all_up_to(
    n: usize,
    answer: &ResultType,
    values: &[ResultType],
    operators: &mut [char],
    is_part2: bool,
) -> bool {
    operators[n] = '*';
    if n > 0 {
        if test_all_up_to(n - 1, answer, values, operators, is_part2) {
            return true;
        }
    } else if evaluate(values, operators, is_part2) == *answer {
        return true;
    }
    operators[n] = '+';
    if n > 0 {
        if test_all_up_to(n - 1, answer, values, operators, is_part2) {
            return true;
        }
    } else if evaluate(values, operators, is_part2) == *answer {
        return true;
    }
    if is_part2 {
        operators[n] = '|';
        if n > 0 {
            if test_all_up_to(n - 1, answer, values, operators, is_part2) {
                return true;
            }
        } else if evaluate(values, operators, is_part2) == *answer {
            return true;
        }
    }
    false
}

fn concatenate(lhs: ResultType, rhs: ResultType) -> ResultType {
    let mut scale = 10;
    loop {
        if rhs < scale {
            break;
        }
        scale *= 10;
    }
    rhs + scale * lhs
}
