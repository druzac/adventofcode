use crate::{aocerror, AOCError, ProblemPart};

use std::cmp;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead, Lines};
use std::str::FromStr;

#[derive(Debug)]
struct ValveNode {
    name: String,
    flow_rate: u64,
    neighbours: Vec<String>,
}

fn trim_suffix(s: &str) -> &str {
    s.trim_end_matches(&[';', ','])
}

type Graph = HashMap<String, ValveNode>;

fn parse_problem<B: BufRead>(lines: Lines<B>) -> Result<Graph, AOCError> {
    let mut m = HashMap::new();
    for raw_line in lines {
        let node = raw_line?.parse::<ValveNode>()?;
        let replaced_val = m.insert(node.name.clone(), node);
        if let Some(ejected_node) = replaced_val {
            return Err(aocerror!(
                "already have an entry for name: {} in graph: {:?}",
                ejected_node.name,
                m
            ));
        }
    }
    Ok(m)
}

impl FromStr for ValveNode {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<_> = s.split(' ').collect();
        let name = words[1].to_string();
        let flow_rate = trim_suffix(
            words[4]
                .split_once('=')
                .ok_or(aocerror!("expected = in string: {}", words[4]))?
                .1,
        )
        .parse::<u64>()?;
        let neighbours: Vec<_> = words
            .iter()
            .skip(9)
            .map(|valve_name| trim_suffix(valve_name).to_string())
            .collect();
        Ok(ValveNode {
            name: name,
            flow_rate: flow_rate,
            neighbours: neighbours,
        })
    }
}

fn neighbours<'a, 'b>(g: &'a Graph, key: &'b str) -> impl Iterator<Item = &'a String> + 'a {
    g.get(key).map_or([].iter(), |node| node.neighbours.iter())
}

fn root_useful_pairs_shortest_path<'a>(start: &str, g: &'a Graph) -> HashMap<&'a str, PathStats> {
    let mut shortest_paths = HashMap::new();
    let mut frontier: VecDeque<(&str, u64)> = VecDeque::new();
    frontier.push_back((start, 0));
    let mut explored: HashSet<&str> = HashSet::new();
    explored.insert(start);
    while let Some((current_name, path_length)) = frontier.pop_front() {
        for neighbour in neighbours(g, current_name) {
            if explored.insert(neighbour.as_str()) {
                explored.insert(current_name);
                let path_length_to_neighbour = path_length + 1;
                frontier.push_back((neighbour, path_length_to_neighbour));
                let neighbour_node = g.get(neighbour).unwrap();
                if neighbour_node.flow_rate > 0 {
                    shortest_paths.insert(
                        neighbour.as_str(),
                        PathStats::new(path_length_to_neighbour, neighbour_node.flow_rate),
                    );
                }
            }
        }
    }
    shortest_paths
}

#[derive(Debug)]
struct PathStats {
    // includes time to turn on valve
    length: u64,
    destination_node_rate: u64,
}

impl PathStats {
    fn new(path_length: u64, destination_node_rate: u64) -> PathStats {
        // add one to account for turning the valve
        PathStats {
            length: path_length + 1,
            destination_node_rate: destination_node_rate,
        }
    }

    fn potential_value(&self, remaining_time: u64) -> u64 {
        if remaining_time <= self.length {
            0
        } else {
            (remaining_time - self.length) * self.destination_node_rate
        }
    }
}

type ShortestPaths<'a> = HashMap<&'a str, HashMap<&'a str, PathStats>>;

fn all_useful_pairs_shortest_path(g: &Graph) -> ShortestPaths {
    let mut shortest_paths = HashMap::new();
    shortest_paths.insert("AA", root_useful_pairs_shortest_path("AA", g));
    for node in g.values() {
        if node.name != "AA" && node.flow_rate > 0 {
            shortest_paths.insert(
                node.name.as_str(),
                root_useful_pairs_shortest_path(node.name.as_str(), g),
            );
        }
    }
    shortest_paths
}

#[derive(Clone, Debug)]
struct EState {
    current_node: String,
    remaining_time: u64,
    accum_pressure: u64,
    seen_nodes: Vec<String>,
}

impl EState {
    fn move_to(&self, next_node: &str, pstats: &PathStats) -> Option<EState> {
        let foo = next_node.to_string();
        if self.seen_nodes.contains(&foo) {
            return None;
        }
        let pvalue = pstats.potential_value(self.remaining_time);
        if pvalue == 0 {
            return None;
        }
        assert!(self.remaining_time >= pstats.length);
        let mut new_estate = self.clone();
        new_estate.current_node = next_node.to_string();
        new_estate.remaining_time -= pstats.length;
        new_estate.accum_pressure += pvalue;
        new_estate.seen_nodes.push(next_node.to_string());
        Some(new_estate)
    }
}

fn problem1(g: Graph) -> u64 {
    let shortest_paths = all_useful_pairs_shortest_path(&g);
    let mut frontier = VecDeque::new();
    frontier.push_back(EState {
        current_node: "AA".to_string(),
        remaining_time: 30,
        accum_pressure: 0,
        seen_nodes: vec!["AA".to_string()],
    });
    let mut best_pressure = 0;
    while let Some(estate) = frontier.pop_front() {
        best_pressure = cmp::max(best_pressure, estate.accum_pressure);
        for (next_name, pstats) in shortest_paths
            .get(estate.current_node.as_str())
            .unwrap()
            .iter()
        {
            if let Some(new_estate) = estate.move_to(next_name, pstats) {
                frontier.push_back(new_estate);
            }
        }
    }
    best_pressure
}

fn intersect_sorted(v1: &[String], v2: &[String]) -> bool {
    let mut i = 0;
    let mut j = 0;
    while i < v1.len() && j < v2.len() {
        match v1[i].cmp(&v2[j]) {
            cmp::Ordering::Equal => return true,
            cmp::Ordering::Less => i += 1,
            cmp::Ordering::Greater => j += 1,
        }
    }
    false
}

fn problem2(g: Graph) -> u64 {
    let mut all_explored_nodes = Vec::new();
    let shortest_paths = all_useful_pairs_shortest_path(&g);
    let mut frontier = VecDeque::new();
    frontier.push_back(EState {
        current_node: "AA".to_string(),
        remaining_time: 26,
        accum_pressure: 0,
        seen_nodes: vec!["AA".to_string()],
    });
    let mut best_single_pressure = 0;
    let mut best_idx = 0;
    while let Some(estate) = frontier.pop_front() {
        for (next_name, pstats) in shortest_paths
            .get(estate.current_node.as_str())
            .unwrap()
            .iter()
        {
            if let Some(new_estate) = estate.move_to(next_name, pstats) {
                frontier.push_back(new_estate);
            }
        }
        if estate.accum_pressure > best_single_pressure {
            best_single_pressure = estate.accum_pressure;
            best_idx = all_explored_nodes.len();
        }
        all_explored_nodes.push(estate);
    }
    for estate in all_explored_nodes.iter_mut() {
        estate.seen_nodes.swap_remove(0);
        estate.seen_nodes.sort();
    }
    // compute greedy solution: best solution for one person and then best solution with remaining nodes.
    let mut best_remaining_for_best_single = 0;
    for estate in &all_explored_nodes {
        if !intersect_sorted(&all_explored_nodes[best_idx].seen_nodes, &estate.seen_nodes) {
            best_remaining_for_best_single =
                cmp::max(estate.accum_pressure, best_remaining_for_best_single);
        }
    }
    // use greedy solution to prune search space.
    let mut i = 0;
    while i < all_explored_nodes.len() {
        if all_explored_nodes[i].accum_pressure < best_remaining_for_best_single {
            all_explored_nodes.swap_remove(i);
        } else {
            i += 1;
        }
    }
    let mut best_pressure = 0;
    for i in 0..all_explored_nodes.len() {
        for j in i + 1..all_explored_nodes.len() {
            let e1 = &all_explored_nodes[i];
            let e2 = &all_explored_nodes[j];
            if !intersect_sorted(&e1.seen_nodes, &e2.seen_nodes) {
                best_pressure = cmp::max(best_pressure, e1.accum_pressure + e2.accum_pressure);
            }
        }
    }
    best_pressure
}

pub fn solve<B: io::BufRead>(part: ProblemPart, br: B) -> Result<(), AOCError> {
    let g = parse_problem(br.lines())?;
    let result = match part {
        ProblemPart::P1 => problem1(g),
        ProblemPart::P2 => problem2(g),
    };
    println!("result: {}", result);
    Ok(())
}
