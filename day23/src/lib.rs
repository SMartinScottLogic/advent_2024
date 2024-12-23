use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = String;

#[derive(Debug, Default)]
pub struct Solution {
    connections: Vec<(String, String)>,
}
impl Solution {
    fn add_connection(&mut self, a: &str, b: &str) {
        self.connections.push((a.to_string(), b.to_string()));
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let (a, b) = line.split_once("-").unwrap();
            solution.add_connection(a, b);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut connected: HashMap<String, HashSet<String>> = HashMap::new();
        for (a, b) in &self.connections {
            connected
                .entry((*a).clone())
                .or_default()
                .insert((*b).clone());
            connected
                .entry((*b).clone())
                .or_default()
                .insert((*a).clone());
        }
        debug!(?connected);
        let mut trios = HashSet::new();
        for (a, n1) in &connected {
            for (c, n2) in &connected {
                if n1.contains(c) {
                    for i in n1.intersection(n2) {
                        let mut trio = vec![a, c, i];
                        trio.sort();
                        debug!(?trio);
                        trios.insert(trio);
                    }
                }
            }
        }
        debug!(?trios);
        let r = format!(
            "{}",
            trios
                .iter()
                .filter(|v| v.iter().any(|c| c.starts_with("t")))
                .count()
        );
        // Implement for problem
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut g: HashMap<String, HashSet<String>> = HashMap::new();
        for (a, b) in &self.connections {
            g.entry(a.to_owned()).or_default().insert(b.clone());
            g.entry(b.to_owned()).or_default().insert(a.clone());
        }

        let clique = utils::graph::bron_kerbosch(&g)
            .into_iter()
            .map(|clique| clique.iter().sorted().join(","))
            .max_by_key(|clique| clique.len())
            .unwrap()
            .to_owned();
        info!(?clique);

        // Implement for problem
        Ok(clique)
    }
}
