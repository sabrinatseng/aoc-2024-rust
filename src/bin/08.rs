use std::collections::{HashMap, HashSet};

use advent_of_code::{get_grid_dimensions, Coord};

advent_of_code::solution!(8);

struct Map {
    dimensions: Coord,
    // Map of { frequency: vec of coordinates where there is an antenna of that frequency }
    antennas: HashMap<char, Vec<Coord>>,
}

impl Map {
    fn check_in_bounds(&self, coord: &Coord) -> bool {
        coord.x < self.dimensions.x && coord.y < self.dimensions.y
    }
}

fn parse(input: &str) -> Map {
    let dimensions = get_grid_dimensions(input);

    let mut antennas: HashMap<char, Vec<Coord>> = HashMap::new();
    for (y, line) in input.lines().rev().enumerate() {
        for (x, frequency) in line.chars().enumerate() {
            if frequency == '.' {
                // no antenna here
                continue;
            }

            antennas
                .entry(frequency)
                .or_default()
                .push(Coord::new(x, y));
        }
    }

    Map {
        dimensions,
        antennas,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);
    let mut antinodes = HashSet::new();

    for (_frequency, antennas) in map.antennas.clone() {
        // For every pair of antennas, they create 2 potential antinodes.
        // Check if each one is in the grid
        for i in 0..antennas.len() {
            for j in (i + 1)..antennas.len() {
                let antenna_1 = antennas[i];
                let antenna_2 = antennas[j];

                let (dx, dy) = antenna_2.diff(&antenna_1);

                // apply (dx, dy) on either side of 1 and 2 to find potential antinodes

                if let Some(antinode_1) = antenna_1.step(-dx, -dy) {
                    if map.check_in_bounds(&antinode_1) {
                        antinodes.insert(antinode_1);
                    }
                }

                if let Some(antinode_2) = antenna_2.step(dx, dy) {
                    if map.check_in_bounds(&antinode_2) {
                        antinodes.insert(antinode_2);
                    }
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse(input);
    let mut antinodes = HashSet::new();

    for (_frequency, antennas) in map.antennas.clone() {
        // Every antenna is an antinode
        antinodes.extend(&antennas);
        // For every pair of antennas, they create a line of potential antinodes.
        // Keep applying the diff until we are outside the grid
        for i in 0..antennas.len() {
            for j in (i + 1)..antennas.len() {
                let antenna_1 = antennas[i];
                let antenna_2 = antennas[j];

                let (dx, dy) = antenna_2.diff(&antenna_1);

                // apply (dx, dy) on either side of 1 and 2 until we are
                // outside the grid to find potential antinodes

                let mut antinode_1 = antenna_1;
                while let Some(antinode) = antinode_1.step(-dx, -dy) {
                    if map.check_in_bounds(&antinode) {
                        antinodes.insert(antinode);
                        antinode_1 = antinode;
                    } else {
                        break;
                    }
                }

                let mut antinode_2 = antenna_2;
                while let Some(antinode) = antinode_2.step(dx, dy) {
                    if map.check_in_bounds(&antinode) {
                        antinodes.insert(antinode);
                        antinode_2 = antinode;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
