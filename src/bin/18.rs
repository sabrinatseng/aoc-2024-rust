use std::collections::HashSet;

use advent_of_code::{Coord, Dimensions};
use itertools::Itertools;

advent_of_code::solution!(18);

fn parse_bytes(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(str::parse)
                .map(|res| res.unwrap_or_else(|_| panic!("Failed to parse ints from {line}")))
                .collect_tuple::<(i64, i64)>()
                .unwrap_or_else(|| panic!("Did not find 2 numbers in {line}"))
                .into()
        })
        .collect::<Vec<_>>()
}

#[derive(Debug)]
struct State {
    pos: Coord,
    visited: HashSet<Coord>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let bytes = parse_bytes(input);

    #[cfg(test)]
    let dimensions = Dimensions::new(7, 7);
    #[cfg(not(test))]
    let dimensions = Dimensions::new(71, 71);

    let start = dimensions.small_corner();
    let end = dimensions.large_corner();

    #[cfg(test)]
    let walls: HashSet<Coord> = HashSet::from_iter(bytes.into_iter().take(12));
    #[cfg(not(test))]
    let walls: HashSet<Coord> = HashSet::from_iter(bytes.into_iter().take(1024));

    let start_state = State {
        pos: start,
        visited: HashSet::new(),
    };

    // BFS
    let mut queue = vec![start_state];

    let mut i = 1;
    loop {
        let mut new_queue = Vec::new();
        let mut new_queue_positions = HashSet::new();

        // Try stepping in each direction
        for State { pos, visited } in queue {
            let new_visited = {
                let mut new_visited = visited.clone();
                new_visited.insert(pos);
                new_visited
            };

            for neighbor in dimensions.get_neighbors(&pos) {
                if walls.contains(&neighbor) {
                    // hit a wall
                    continue;
                }

                if visited.contains(&neighbor) {
                    // already visited this spot
                    continue;
                }

                if new_queue_positions.contains(&neighbor) {
                    // this neighbor is already in the new queue
                    continue;
                }

                if neighbor == end {
                    // reached the end
                    return Some(i as u32);
                }

                let new_state = State {
                    pos: neighbor,
                    visited: new_visited.clone(),
                };
                new_queue.push(new_state);
                new_queue_positions.insert(neighbor);
            }
        }

        queue = new_queue;
        i += 1;
    }
}

pub fn part_two(input: &str) -> Option<String> {
    // Search for a path of walls from the left or bottom border to the right or top border.
    // If a path can be found, then the end is not reachable.
    let bytes = parse_bytes(input);

    #[cfg(test)]
    let dimensions = Dimensions::new(7, 7);
    #[cfg(not(test))]
    let dimensions = Dimensions::new(71, 71);

    // We know a path exists for 12/1024 so start searching there
    #[cfg(test)]
    let i = 12;
    #[cfg(not(test))]
    let i = 1024;

    let mut walls: HashSet<Coord> = HashSet::from_iter(bytes.iter().copied().take(i));

    let starts: HashSet<Coord> = dimensions
        .left_borders()
        .chain(dimensions.top_borders())
        .collect();
    let ends: HashSet<Coord> = dimensions
        .right_borders()
        .chain(dimensions.bottom_borders())
        .collect();

    // Compute and keep track of a set of walls reachable from the left or bottom borders using DFS.
    // For each new wall that is added, we only need to recompute the "reachable" walls if
    // the new wall is a neighbor of an existing reachable wall.
    let mut reachable: HashSet<Coord> = walls.intersection(&starts).copied().collect();

    for (i, new_wall) in bytes.into_iter().enumerate().skip(i) {
        // Add the wall
        walls.insert(new_wall);

        // Check if we need to recompute the "reachable" walls
        let recompute = if i == walls.len() {
            // For the first iteration, we need to compute all reachable
            true
        } else {
            let mut new_wall_neighbors = dimensions
                .get_neighbors(&new_wall)
                .chain(dimensions.get_diagonal_neighbors(&new_wall));

            // We only need to recompute if this wall is attached to the current "reachable" set,
            // or if it is a new starting wall
            new_wall_neighbors.any(|c| reachable.contains(&c)) || starts.contains(&new_wall)
        };

        if !recompute {
            continue;
        }

        // Recompute reachable walls
        reachable.insert(new_wall);

        loop {
            let mut new_reachable = HashSet::new();
            for wall in reachable.iter() {
                // Check all neighbors of current reachable walls
                for neighbor in dimensions
                    .get_neighbors(wall)
                    .chain(dimensions.get_diagonal_neighbors(wall))
                {
                    if !walls.contains(&neighbor) {
                        // This neighbor is not a wall so it doesn't contribute to our path
                        continue;
                    }

                    if reachable.contains(&neighbor) || new_reachable.contains(&neighbor) {
                        // this neighbor is already reachable
                        continue;
                    }

                    new_reachable.insert(neighbor);
                }
            }

            if new_reachable.is_empty() {
                // No new walls are reachable at this depth, we can stop searching
                break;
            }

            reachable.extend(new_reachable);
        }

        if reachable.intersection(&ends).next().is_some() {
            // One of the ends (right/top border) is reachable, so we have found the solution
            print_grid(&dimensions, &reachable, 'R');
            print_grid(&dimensions, &walls, 'W');
            let output = format!("{},{}", new_wall.x, new_wall.y);
            return Some(output);
        }
    }

    None
}

// Print a grid for debugging
#[allow(dead_code)]
fn print_grid(dimensions: &Dimensions, set: &HashSet<Coord>, c: char) {
    for y in 0..dimensions.y {
        for x in 0..dimensions.x {
            if set.contains(&Coord::new(x as i64, y as i64)) {
                print!("{c}");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
