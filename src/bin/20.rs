use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code::{parse_maze, Coord, Dimensions, Maze};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    part_one_inner(input, 100)
}

// Threshold = number of picoseconds that must be saved in order to count the cheat
fn part_one_inner(input: &str, threshold: u32) -> Option<u32> {
    let maze = parse_maze(input);
    let dimensions = Dimensions::from_input(input);

    // Find distance from start and end for each non-wall square
    let distance_from_start = distance_from_node(&maze, maze.start);
    let distance_from_end = distance_from_node(&maze, maze.end);

    let shortest_path_without_cheating = distance_from_start.get(&maze.end).unwrap();

    // Count cheats that save at least threshold picoseconds
    let mut count = 0;

    // For each non-wall square, look for length-2 paths ignoring walls (i.e. cheats).
    // total length of path = distance_from_start[node] + 2 + distance_from_end[cheat_end]
    for node in distance_from_start.keys().copied() {
        for cheat_start in dimensions.get_neighbors(&node) {
            for cheat_end in dimensions.get_neighbors(&cheat_start) {
                if maze.walls.contains(&cheat_end) {
                    // Must get back on the track at the end of the cheat
                    continue;
                }

                if cheat_end == node {
                    // If we end up at the same node after the cheat we haven't
                    // saved any time
                    continue;
                }

                let dist = distance_from_start
                    .get(&node)
                    .expect("node not in distance_from_start")
                    + 2
                    + distance_from_end
                        .get(&cheat_end)
                        .expect("cheat_end not in distance_from_end");
                if dist <= shortest_path_without_cheating.saturating_sub(threshold) {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

// Find the shortest distance from node to each other node in the maze
fn distance_from_node(maze: &Maze, node: Coord) -> HashMap<Coord, u32> {
    // BFS state object
    #[derive(Clone, Debug)]
    struct State {
        pos: Coord,
        path: HashSet<Coord>,
    }

    let mut distances = HashMap::new();
    let start_state = State {
        pos: node,
        path: HashSet::new(),
    };

    // BFS
    let mut queue = VecDeque::from_iter([start_state]);
    while let Some(State { pos, path }) = queue.pop_front() {
        // insert if not present
        distances.entry(pos).or_insert_with(|| path.len() as u32);

        let new_path = {
            let mut new_path = path.clone();
            new_path.insert(pos);
            new_path
        };

        for neighbor in pos.get_neighbors() {
            if maze.walls.contains(&neighbor) {
                // hit a wall
                continue;
            }

            if path.contains(&neighbor) {
                // the path already contains this node, but the shortest path
                // should not contain the same node twice
                continue;
            }

            // Add to the queue
            queue.push_back(State {
                pos: neighbor,
                path: new_path.clone(),
            });
        }
    }

    distances
}

pub fn part_two(input: &str) -> Option<u32> {
    part_two_inner(input, 100)
}

// Threshold = number of picoseconds that must be saved in order to count the cheat
fn part_two_inner(input: &str, threshold: u32) -> Option<u32> {
    let maze = parse_maze(input);
    let dimensions = Dimensions::from_input(input);

    // Find distance from start and end for each non-wall square
    let distance_from_start = distance_from_node(&maze, maze.start);
    let distance_from_end = distance_from_node(&maze, maze.end);

    let shortest_path_without_cheating = distance_from_start.get(&maze.end).unwrap();

    // Count cheats that save at least threshold picoseconds
    let mut count = 0;

    // Each possible cheat (cheat_start, cheat_end) has a total path length of
    // distance_from_start[cheat_start] + cheat_length + distance_from_end[cheat_end].
    // For each possible cheat_start, check all possible cheat_ends within cheat_length 20
    // where cheat_length = manhattan_distance(cheat_start, cheat_end).
    for cheat_start in distance_from_start.keys() {
        for dx in -20..=20_i64 {
            let remaining_cheat_len = 20 - dx.abs();
            for dy in -remaining_cheat_len..=remaining_cheat_len {
                let cheat_len = dx.abs() + dy.abs();
                let cheat_end = cheat_start.step(dx, dy);
                if !dimensions.in_bounds(&cheat_end) {
                    // cheat end is not in the maze
                    continue;
                }
                if maze.walls.contains(&cheat_end) {
                    // cheat must end on the track
                    continue;
                }

                let dist = distance_from_start
                    .get(cheat_start)
                    .expect("cheat_start not in distance_from_start")
                    + cheat_len as u32
                    + distance_from_end
                        .get(&cheat_end)
                        .expect("cheat_end not in distance_from_end");

                if dist <= shortest_path_without_cheating.saturating_sub(threshold) {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let run_with_threshold = |threshold| {
            part_one_inner(
                &advent_of_code::template::read_file("examples", DAY),
                threshold,
            )
        };

        assert_eq!(run_with_threshold(100), Some(0));

        // Based on the example in the problem
        // counts are cumulative
        assert_eq!(run_with_threshold(64), Some(1));
        assert_eq!(run_with_threshold(40), Some(2));
        assert_eq!(run_with_threshold(38), Some(3));
        assert_eq!(run_with_threshold(36), Some(4));
        assert_eq!(run_with_threshold(20), Some(5));
        assert_eq!(run_with_threshold(12), Some(8));
        assert_eq!(run_with_threshold(10), Some(10));
        assert_eq!(run_with_threshold(8), Some(14));
        assert_eq!(run_with_threshold(6), Some(16));
        assert_eq!(run_with_threshold(4), Some(30));
        assert_eq!(run_with_threshold(2), Some(44));
    }

    #[test]
    fn test_part_two() {
        let run_with_threshold = |threshold| {
            part_two_inner(
                &advent_of_code::template::read_file("examples", DAY),
                threshold,
            )
        };

        assert_eq!(run_with_threshold(100), Some(0));

        // Based on the example in the problem
        // counts are cumulative
        assert_eq!(run_with_threshold(76), Some(3));
        assert_eq!(run_with_threshold(74), Some(7));
        assert_eq!(run_with_threshold(72), Some(29));
        assert_eq!(run_with_threshold(70), Some(41));
        assert_eq!(run_with_threshold(68), Some(55));
        assert_eq!(run_with_threshold(66), Some(67));
        assert_eq!(run_with_threshold(64), Some(86));
        assert_eq!(run_with_threshold(62), Some(106));
        assert_eq!(run_with_threshold(60), Some(129));
        assert_eq!(run_with_threshold(58), Some(154));
        assert_eq!(run_with_threshold(56), Some(193));
        assert_eq!(run_with_threshold(54), Some(222));
        assert_eq!(run_with_threshold(52), Some(253));
        assert_eq!(run_with_threshold(50), Some(285));
    }
}
