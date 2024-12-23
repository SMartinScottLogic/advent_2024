use std::cmp::{Eq, PartialOrd};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::Iterator;
use std::marker::Copy;
use std::ops::Add;
use tracing::debug;

fn breadth_first_search_build_path<'a, N>(
    mut position: &'a N,
    path_fragments: &'a HashMap<N, N>,
) -> VecDeque<N>
where
    N: Debug + PartialEq + Eq + Hash + Clone,
{
    debug!(
        path_fragments = debug(&path_fragments),
        position = debug(position),
        "build path"
    );
    let mut total_path = VecDeque::new();
    while let Some(current) = path_fragments.get(position) {
        total_path.push_front(position.clone());
        position = current;
    }
    total_path
}

pub fn breadth_first_search<'a, N, IE, GN>(
    start: N,
    get_neighbours: GN,
    is_end: IE,
) -> Option<(N, VecDeque<N>)>
where
    N: Debug + PartialEq + Eq + Hash + Clone + 'a,
    IE: Fn(&N) -> bool,
    GN: Fn(&N) -> Vec<N>,
{
    let mut have_seen = HashSet::new();
    have_seen.insert(start.clone());
    let mut queue = VecDeque::new();
    queue.push_back(start.clone());
    let mut path_fragments = HashMap::new();

    while let Some(current) = queue.pop_front() {
        debug!(
            current = debug(&current),
            queue = debug(&queue),
            have_seen = debug(&have_seen),
            "popped"
        );
        if is_end(&current) {
            return Some((
                current.clone(),
                breadth_first_search_build_path(&current, &path_fragments),
            ));
        }
        for neighbour in get_neighbours(&current) {
            if have_seen.contains(&neighbour) {
                debug!(
                    current = debug(&current),
                    neighbour = debug(&neighbour),
                    "have seen"
                );
                continue;
            }
            have_seen.insert(neighbour.clone());
            queue.push_back(neighbour.clone());
            path_fragments.insert(neighbour.clone(), current.clone());
        }
    }
    None
}

pub fn dijkstra<N, IS, IE, GN, NEIGH, R>(
    nodes: &Vec<N>,
    initial_score: IS,
    get_neighbours: GN,
    is_end: IE,
) -> Option<R>
where
    IS: Fn(&N) -> Option<R>,
    GN: Fn(&N) -> NEIGH,
    IE: Fn(&N) -> bool,
    N: Debug + Eq + Copy + Hash,
    R: Debug + PartialOrd + Copy + Add<Output = R> + HasOne,
    NEIGH: Iterator<Item = N>,
{
    let mut scores = HashMap::new();
    for node in nodes {
        if let Some(s) = initial_score(node) {
            scores.insert(*node, s);
        }
    }
    let mut visited = HashSet::new();
    let result = loop {
        // Find smallest, unvisited
        let mut bestnode = None;
        let mut bestscore = None;
        for (node, score) in scores.iter() {
            if !visited.contains(node) {
                match bestscore {
                    None => {
                        bestnode = Some(node.to_owned());
                        bestscore = Some(score.to_owned());
                    }
                    Some(s) if s > *score => {
                        bestnode = Some(*node);
                        bestscore = Some(score.to_owned());
                    }
                    Some(_) => {}
                }
            }
        }
        if bestnode.is_none() {
            break None;
        }
        let bestnode = bestnode.unwrap();
        let bestscore = bestscore.unwrap();
        visited.insert(bestnode);
        if is_end(&bestnode) {
            break Some(bestscore);
        }
        let neighbours = get_neighbours(&bestnode);
        for neighbour in neighbours {
            let n = neighbour;
            let score = scores.entry(n).or_insert(bestscore + R::one());
            if *score > bestscore + R::one() {
                *score = bestscore + R::one();
            }
        }
    };
    debug!("{:?}", scores);
    result
}

pub trait HasOne {
    fn one() -> Self;
}

impl HasOne for i64 {
    fn one() -> Self {
        1
    }
}

type Graph<'a, N> = HashMap<N, HashSet<N>>;
type NodeSet<N> = HashSet<N>;

/**
https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm

In computer science, the Bron–Kerbosch algorithm is an enumeration algorithm for finding all
maximal cliques in an undirected graph. That is, it lists all subsets of vertices with the
two properties that each pair of vertices in one of the listed subsets is connected by an edge,
and no listed subset can have any additional vertices added to it while preserving its complete
connectivity. The Bron–Kerbosch algorithm was designed by Dutch scientists Coenraad Bron and
Joep Kerbosch, who published its description in 1973.
*/
pub fn bron_kerbosch<N>(graph: &Graph<N>) -> Vec<NodeSet<N>>
where
    N: Clone + Eq + Hash,
{
    let mut cliques = Vec::new();

    let mut r = HashSet::new();
    let x = HashSet::new();
    //let p: NodeSet<usize> = (0..graph.len()).collect();
    let p = graph.keys().cloned().collect();

    bron_kerbosch1(graph, &mut cliques, &mut r, p, x);

    cliques
}

fn bron_kerbosch1<N>(
    graph: &Graph<N>,
    cliques: &mut Vec<NodeSet<N>>,
    r: &mut NodeSet<N>,
    p: NodeSet<N>,
    mut x: NodeSet<N>,
) where
    N: Clone + Eq + Hash,
{
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
    let pivot = p.union(&x).max_by_key(|v| graph[*v].len()).unwrap().clone();

    for v in p.difference(&graph[&pivot]) {
        r.insert(v.to_owned());
        let p1: NodeSet<N> = p_clone.intersection(&graph[v]).cloned().collect();
        let x1: NodeSet<N> = x.intersection(&graph[v]).cloned().collect();
        bron_kerbosch1(graph, cliques, r, p1, x1);
        r.remove(v);

        p_clone.remove(v);
        x.insert(v.to_owned());
    }
}
