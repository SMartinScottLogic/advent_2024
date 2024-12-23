use std::{collections::{HashMap, HashSet}, io::{BufRead, BufReader}};
use itertools::Itertools;
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
            connected.entry((*a).clone()).or_default().insert((*b).clone());
            connected.entry((*b).clone()).or_default().insert((*a).clone());
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
        let r = format!("{}", trios.iter().filter(|v| v.iter().any(|c| c.starts_with("t"))).count());
        // Implement for problem
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let machines = self.connections.iter().fold(HashSet::new(), |mut acc, (a, b)| {
            acc.insert(a);
            acc.insert(b);
            acc
        }).iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, m)| {
            acc.insert((**m).clone(), i);
            acc
        });

        let mut neighbours = vec![HashSet::new(); machines.len()];
        for (a, b) in &self.connections {
            let a_id = *machines.get(a).unwrap();
            let b_id = *machines.get(b).unwrap();
            neighbours[a_id].insert(b_id);
            neighbours[b_id].insert(a_id);
        }
        let cliques = bron_kerbosch(&neighbours);
        debug!(?cliques);

        let r_machines = machines.iter().fold(HashMap::new(), |mut acc, (name, id)| {
            acc.insert(*id, name.to_owned());
            acc
        });
        let cliques = cliques.into_iter().map(|clique| {
            clique.iter().map(|id| r_machines[id].clone()).sorted().join(",")
        }).collect::<Vec<_>>();
        let clique = cliques.iter().max_by_key(|n| n.len()).unwrap().to_owned();
        debug!(?clique);

        // Implement for problem
        Ok(clique)
    }
}

type Graph<'a> = &'a [HashSet<usize>];
type NodeSet = HashSet<usize>;

fn bron_kerbosch(graph: Graph) -> Vec<NodeSet> {
    let mut cliques = Vec::new();

    let mut r = HashSet::new();
    let x = HashSet::new();
    let p: NodeSet = (0..graph.len()).collect();

    bron_kerbosch1(graph, &mut cliques, &mut r, p, x);

    cliques
}

fn bron_kerbosch1(
    graph: Graph,
    cliques: &mut Vec<NodeSet>,
    r: &mut NodeSet,
    p: NodeSet,
    mut x: NodeSet,
) {
    if p.is_empty() && x.is_empty() {
        if cliques.is_empty() {
            cliques.push(r.clone());
            return;
        }

        let cur = cliques.first().unwrap().len();
        if cur < r.len() {
            cliques.clear();
        }
        if cur <= r.len() {
            cliques.push(r.clone())
        }
        return;
    }

    let mut p_clone = p.clone();
    let pivot = *p.union(&x).max_by_key(|&&v| graph[v].len()).unwrap();

    for &v in p.difference(&graph[pivot]) {
        r.insert(v);
        let p1: NodeSet = p_clone.intersection(&graph[v]).cloned().collect();
        let x1: NodeSet = x.intersection(&graph[v]).cloned().collect();
        bron_kerbosch1(graph, cliques, r, p1, x1);
        r.remove(&v);

        p_clone.remove(&v);
        x.insert(v);
    }
}
