use std::collections::HashSet;

use advent_of_code::{Dimensions, Grid};

advent_of_code::solution!(10);

fn parse(input: &str) -> Grid<u8> {
    let dimensions = Dimensions::from_input(input);

    let values = input
        .lines()
        .rev()
        .map(|line| line.chars().map(|c| (c as u8) - 48).collect())
        .collect();

    Grid { dimensions, values }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);

    let mut score = 0;

    for trailhead in grid.positions_of(&0) {
        // BFS style search
        // Start with the trailhead
        let mut pending = HashSet::new();
        pending.insert(trailhead);

        for i in 1..=9 {
            // Calculate all reachable nodes with value i
            let mut new_pending = HashSet::new();
            for position in pending {
                for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let coord = position.step(dx, dy);
                    if grid.get(&coord) == Some(i) {
                        new_pending.insert(coord);
                    }
                }
            }

            pending = new_pending;
        }

        // At the end of this loop we should have positions of all the reachable 9s
        score += pending.len();
    }

    Some(score as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse(input);

    let mut score = 0;

    for trailhead in grid.positions_of(&0) {
        // BFS style search
        // Start with the trailhead
        // Store the latest position of the trail - note there can be duplicates if there are multiple
        // distinct paths to get there
        let mut pending = vec![trailhead];

        for i in 1..=9 {
            // Calculate all reachable trails with length i
            let mut new_pending = Vec::new();
            for position in pending {
                for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let coord = position.step(dx, dy);
                    if grid.get(&coord) == Some(i) {
                        new_pending.push(coord);
                    }
                }
            }

            pending = new_pending;
        }

        // At the end of this loop we should have all the distinct reachable trails
        score += pending.len();
    }

    Some(score as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
