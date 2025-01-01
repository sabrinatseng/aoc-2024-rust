use std::collections::{BinaryHeap, HashMap, HashSet};

use advent_of_code::{parse_maze, Coord, Direction, Maze};

advent_of_code::solution!(16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    pos: Coord,
    dir: Direction,
}

impl State {
    fn start_state(maze: &Maze) -> State {
        State {
            pos: maze.start,
            dir: Direction::Right,
        }
    }

    fn step(&self) -> Self {
        Self {
            pos: self.pos.step_in_direction(self.dir),
            dir: self.dir,
        }
    }

    fn turn_left(&self) -> Self {
        State {
            pos: self.pos,
            dir: self.dir.turn_left(),
        }
    }

    fn turn_right(&self) -> Self {
        State {
            pos: self.pos,
            dir: self.dir.turn_right(),
        }
    }
}

#[test]
fn test_state() {
    let state = State {
        pos: Coord::new(10, 10),
        dir: Direction::Right,
    };

    assert_eq!(state.step().pos, Coord::new(11, 10));
    assert_eq!(state.step().step().pos, Coord::new(12, 10));
    assert_eq!(state.turn_left().pos, Coord::new(10, 10));
    assert_eq!(state.turn_left().dir, Direction::Up);
    assert_eq!(state.turn_right().dir, Direction::Down);
    assert_eq!(state.turn_left().step().pos, Coord::new(10, 11));
    assert_eq!(state.turn_right().step().pos, Coord::new(10, 9));
}

// Helper struct to use in BinaryHeap to make it a priority queue
#[derive(Clone, Copy, PartialEq, Eq)]
struct PqState {
    score_so_far: usize, // score to get to the current state
    state: State,
}

// Make it a min-heap so we can explore lower cost paths first
impl Ord for PqState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score_so_far.cmp(&self.score_so_far)
    }
}

impl PartialOrd for PqState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_lowest_score(maze: &Maze) -> Option<usize> {
    let start_pq_state = PqState {
        score_so_far: 0,
        state: State::start_state(maze),
    };

    // Use heap as a priority queue
    let mut queue = BinaryHeap::new();
    queue.push(start_pq_state);

    let mut visited = HashSet::new();

    // Start from the lowest score so far
    while let Some(PqState {
        score_so_far,
        state,
    }) = queue.pop()
    {
        if state.pos == maze.end {
            // We've reached the end, return the score
            return Some(score_so_far);
        }

        if visited.contains(&state) {
            // we have already checked this state and didn't find a solution
            continue;
        }
        visited.insert(state);

        // Try stepping in each direction
        for new_state in [
            state.step(),
            state.turn_left().step(),
            state.turn_right().step(),
        ] {
            if maze.walls.contains(&new_state.pos) {
                // hit a wall
                continue;
            }

            if visited.contains(&new_state) {
                // we have already checked this state and didn't find a solution
                continue;
            }

            let cost = if new_state.dir == state.dir {
                1
            } else {
                1001 // we turned and stepped so 1000 + 1
            };

            // Add to the priority queue
            queue.push(PqState {
                score_so_far: score_so_far + cost,
                state: new_state,
            });
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let maze = parse_maze(input);

    let score = find_lowest_score(&maze).unwrap();

    Some(score as u32)
}

// Helper struct to use in BinaryHeap to make it a priority queue
#[derive(Clone, PartialEq, Eq)]
struct PqState2 {
    score_so_far: usize, // score to get to the current state
    state: State,
    path: HashSet<Coord>,
}

// Make it a min-heap so we can explore lower cost paths first
impl Ord for PqState2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score_so_far.cmp(&self.score_so_far)
    }
}

impl PartialOrd for PqState2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_lowest_score_seats(maze: &Maze) -> HashSet<Coord> {
    let start_pq_state = PqState2 {
        score_so_far: 0,
        state: State::start_state(maze),
        path: HashSet::from([maze.start]),
    };

    // Use heap as a priority queue
    let mut queue = BinaryHeap::new();
    queue.push(start_pq_state);

    let mut best_score = None;

    let mut best_seats = HashSet::new();

    // Optimize by pruning the search if we have already found a lower score
    // way to reach this node
    let mut min_score_to_node = HashMap::new();

    // Start from the lowest score so far
    while let Some(PqState2 {
        score_so_far,
        state,
        path,
    }) = queue.pop()
    {
        if best_score.is_some() && score_so_far > best_score.unwrap() {
            // All other paths are longer than the best path, so we can stop searching
            break;
        }

        if state.pos == maze.end {
            // We've reached the end, set the best score
            best_score = Some(score_so_far);
            best_seats = best_seats.union(&path).cloned().collect();
            continue;
        }

        if min_score_to_node.contains_key(&state)
            && *min_score_to_node.get(&state).unwrap() < score_so_far
        {
            // We have already explored this state with a lower score so this can't be the best path
            continue;
        }
        min_score_to_node.insert(state, score_so_far);

        // Try stepping in each direction
        for new_state in [
            state.step(),
            state.turn_left().step(),
            state.turn_right().step(),
        ] {
            if maze.walls.contains(&new_state.pos) {
                // hit a wall
                continue;
            }

            let cost = if new_state.dir == state.dir {
                1
            } else {
                1001 // we turned and stepped so 1000 + 1
            };

            let new_path = {
                let mut new_path = path.clone();
                new_path.insert(new_state.pos);
                new_path
            };

            // Add to the priority queue
            queue.push(PqState2 {
                score_so_far: score_so_far + cost,
                state: new_state,
                path: new_path,
            });
        }
    }

    best_seats
}

pub fn part_two(input: &str) -> Option<u32> {
    let maze = parse_maze(input);

    let lowest_score_seats = find_lowest_score_seats(&maze);

    Some(lowest_score_seats.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}
