use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(23);

fn parse(input: &str) -> UndirectedGraph {
    let edges = input.lines().map(parse_edge).collect();
    UndirectedGraph::from_edges(edges)
}

fn parse_edge(input: &str) -> (String, String) {
    // format: {start_node}-{end_node} where nodes are always 2 chars
    input
        .split('-')
        .map(ToString::to_string)
        .collect_tuple()
        .unwrap_or_else(|| panic!("Failed to parse {input} as edge"))
}

pub fn part_one(input: &str) -> Option<u32> {
    let graph = parse(input);

    let mut three_interconnected = HashSet::new();
    for (n1, neighbors1) in graph.edges.iter() {
        // Only check from nodes that start with t
        if !n1.starts_with("t") {
            continue;
        }

        for n2 in neighbors1 {
            // Any other node that is a neighbor of both n1 and n2
            // creates an interconnected component
            for n3 in graph.get_neighbors(n2) {
                if neighbors1.contains(n3) {
                    three_interconnected.insert(sort_nodes(n1, n2, n3));
                }
            }
        }
    }

    Some(three_interconnected.len() as u32)
}

// Sort alphabetically
fn sort_nodes(n1: &str, n2: &str, n3: &str) -> (String, String, String) {
    let mut arr = [n1, n2, n3];
    arr.sort();
    arr.into_iter()
        .map(ToString::to_string)
        .collect_tuple()
        .unwrap()
}

struct UndirectedGraph {
    edges: HashMap<String, HashSet<String>>,
}

impl UndirectedGraph {
    fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    fn from_edges(edges: Vec<(String, String)>) -> Self {
        let mut graph = Self::new();

        for (start, end) in edges {
            graph.insert_edge(start, end);
        }

        graph
    }

    fn insert_edge(&mut self, start_node: String, end_node: String) {
        self.edges
            .entry(start_node.clone())
            .or_default()
            .insert(end_node.clone());
        self.edges.entry(end_node).or_default().insert(start_node);
    }

    fn get_neighbors(&self, node: &str) -> &HashSet<String> {
        self.edges
            .get(node)
            .unwrap_or_else(|| panic!("Node {node} not in graph"))
    }

    fn nodes(&self) -> impl Iterator<Item = String> + '_ {
        self.edges.keys().cloned()
    }
}

// Recursively search and maintain state of what nodes could be contained in the set.
// Maintain the invariant that could_contain only contains nodes which are neighbors
// of all nodes in contains.
// For each node that could be contained, try including or excluding it.
// based on the Bron–Kerbosch algorithm (https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm)
fn find_largest_interconnected_set(
    graph: &UndirectedGraph,
    contains: HashSet<String>,
    could_contain: VecDeque<String>,
) -> HashSet<String> {
    if could_contain.is_empty() {
        return contains;
    }

    let mut could_contain = could_contain.clone();
    let next = could_contain.pop_front().unwrap();

    // If we include next:
    let includes = {
        let mut new_contains = contains.clone();
        new_contains.insert(next.clone());

        // Filter out remaining could_contain by next's neighbors
        let next_neighbors = graph.get_neighbors(&next);
        let new_could_contain = could_contain
            .clone()
            .into_iter()
            .filter(|node| next_neighbors.contains(node))
            .collect();

        find_largest_interconnected_set(graph, new_contains, new_could_contain)
    };

    // If we don't include next:
    let excludes = find_largest_interconnected_set(graph, contains, could_contain);

    // Return larger set
    if includes.len() >= excludes.len() {
        includes
    } else {
        excludes
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let graph = parse(input);

    let largest_interconnected_set =
        find_largest_interconnected_set(&graph, HashSet::new(), graph.nodes().collect());

    Some(get_password(largest_interconnected_set))
}

fn get_password(set: HashSet<String>) -> String {
    let mut v = Vec::from_iter(set);
    v.sort();
    v.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
