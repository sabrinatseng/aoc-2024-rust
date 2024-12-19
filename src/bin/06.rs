use std::collections::HashSet;

use advent_of_code::{Coord, Dimensions, Direction};
use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Clone)]
struct Map {
    dimensions: Dimensions,
    curr_pos: Coord,
    curr_dir: Direction,
    obstructions: HashSet<Coord>,
}

impl Map {
    fn new(dimensions: Dimensions, start: Coord, obstructions: HashSet<Coord>) -> Self {
        Self {
            dimensions,
            curr_pos: start,
            curr_dir: Direction::Up,
            obstructions,
        }
    }

    // Take a step and return the new position, or None if we have exited the map
    // Note: turning is considered a step, even though we remain in the same position
    fn step(&mut self) -> Option<Coord> {
        // Try to step in the current direction
        let (dx, dy) = self.curr_dir.to_dx_dy();
        let coord = self.curr_pos.step(dx, dy);

        // exited the map
        if !self.dimensions.in_bounds(&coord) {
            return None;
        }

        if !self.obstructions.contains(&coord) {
            // no obstruction
            self.curr_pos = coord;
            Some(coord)
        } else {
            // hit an obstruction, so we should turn and stay in the same position
            self.curr_dir = self.curr_dir.turn_right();
            Some(self.curr_pos)
        }
    }

    fn add_obstruction(&mut self, coord: Coord) {
        self.obstructions.insert(coord);
    }
}

fn parse(input: &str) -> Map {
    let mut start = None;
    let mut obstructions = HashSet::new();

    let dimensions = Dimensions::from_input(input);

    // Reverse the lines since our coordinate system has (0, 0) in the bottom left
    for (y, line) in input.lines().rev().enumerate() {
        if let Some(x) = line.chars().position(|c| c == '^') {
            start = Some(Coord::new(x as i64, y as i64));
        }

        for x in line.chars().positions(|c| c == '#') {
            obstructions.insert(Coord::new(x as i64, y as i64));
        }
    }

    let start = start.expect("Did not find start character ^");
    Map::new(dimensions, start, obstructions)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse(input);

    let mut visited = HashSet::new();
    visited.insert(map.curr_pos); // include the starting position

    while let Some(coord) = map.step() {
        visited.insert(coord);
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse(input);

    let mut loop_positions = 0;

    let new_obstructions_to_check = {
        let mut map = map.clone();
        let mut visited = HashSet::new();
        visited.insert(map.curr_pos); // include the starting position

        while let Some(coord) = map.step() {
            visited.insert(coord);
        }

        visited
    };

    // Brute force - try all the new positions for an obstruction
    // Only check positions that were visited in part 1, as these can affect the path
    for coord in new_obstructions_to_check {
        if map.obstructions.contains(&coord) {
            // Already an obstruction here so we can't add one
            continue;
        }

        let mut map = map.clone();
        map.add_obstruction(coord);

        // Visited states (pos and dir)
        let mut visited = HashSet::new();
        visited.insert((map.curr_pos, map.curr_dir)); // include the starting position

        while let Some(coord) = map.step() {
            let state = (coord, map.curr_dir);
            if visited.contains(&state) {
                // We have looped
                loop_positions += 1;
                break;
            } else {
                visited.insert(state);
            }
        }
    }

    Some(loop_positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
