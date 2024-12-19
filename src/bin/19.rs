use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(19);

// Return (patterns, designs)
fn parse(input: &str) -> (HashSet<String>, Vec<String>) {
    let (patterns, designs) = input
        .split("\n\n")
        .collect_tuple()
        .expect("Expected two blocks of text in input");

    let patterns = patterns
        .trim()
        .split(", ")
        .map(ToString::to_string)
        .collect();
    let designs = designs.lines().map(ToString::to_string).collect();

    (patterns, designs)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (patterns, designs) = parse(input);

    let mut memo = HashMap::new();
    let possible = designs
        .into_iter()
        .filter(|design| design_is_possible(patterns.clone(), design.clone(), &mut memo))
        .count();

    Some(possible as u32)
}

fn design_is_possible(
    patterns: HashSet<String>,
    design: String,
    memo: &mut HashMap<String, bool>,
) -> bool {
    // base cases
    if design.is_empty() {
        return true;
    }
    if let Some(cached) = memo.get(&design) {
        return *cached;
    }

    // Try to match the start of the design
    // Start from the end to try to greedily match as much as possible
    for i in (1..=design.len()).rev() {
        if patterns.contains(&design[..i])
            && design_is_possible(patterns.clone(), design[i..].to_string(), memo)
        {
            memo.insert(design.clone(), true);
            return true;
        }
    }

    memo.insert(design.clone(), false);
    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let (patterns, designs) = parse(input);

    let mut memo = HashMap::new();
    let ways = designs
        .into_iter()
        .map(|design| design_ways(patterns.clone(), design.clone(), &mut memo))
        .sum::<usize>();

    Some(ways as u64)
}

fn design_ways(
    patterns: HashSet<String>,
    design: String,
    memo: &mut HashMap<String, usize>,
) -> usize {
    // base cases
    if design.is_empty() {
        return 1;
    }
    if let Some(cached) = memo.get(&design) {
        return *cached;
    }

    // Try to match the start of the design
    let mut ways = 0;
    for i in 1..=design.len() {
        if patterns.contains(&design[..i]) {
            ways += design_ways(patterns.clone(), design[i..].to_string(), memo);
        }
    }

    memo.insert(design.clone(), ways);
    ways
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
