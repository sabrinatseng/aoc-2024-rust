use std::collections::{HashSet, VecDeque};

use advent_of_code::{get_grid_dimensions, Coord};

advent_of_code::solution!(12);

struct Map {
    dimensions: Coord,
    vals: Vec<Vec<char>>,
}

impl Map {
    fn get_neighbors<'a>(&'a self, coord: &'a Coord) -> impl Iterator<Item = Coord> + 'a {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter_map(|(dx, dy)| coord.step(dx, dy))
            .filter(|coord| self.is_in_bounds(coord))
    }

    fn is_in_bounds(&self, coord: &Coord) -> bool {
        coord.x < self.dimensions.x && coord.y < self.dimensions.y
    }

    fn get(&self, coord: &Coord) -> Option<char> {
        self.vals.get(coord.y)?.get(coord.x).copied()
    }

    fn find_regions(&self) -> Vec<Region> {
        let mut visited = HashSet::new();
        let mut regions = Vec::new();

        for x in 0..self.dimensions.x {
            for y in 0..self.dimensions.y {
                let coord = Coord::new(x, y);
                if visited.contains(&coord) {
                    continue;
                }

                let mut region = Region::new();
                region.insert(coord);
                let val = self.get(&coord).unwrap();

                let mut to_visit = VecDeque::new();
                to_visit.push_back(coord);

                // Use "flood fill" technique to find all coords in this region
                while let Some(next) = to_visit.pop_front() {
                    for neighbor in self.get_neighbors(&next) {
                        if visited.contains(&neighbor) {
                            continue;
                        }

                        if self.get(&neighbor) == Some(val) {
                            // this neighbor is part of the region
                            region.insert(neighbor);
                            visited.insert(neighbor);
                            to_visit.push_back(neighbor);
                        }
                    }
                }

                regions.push(region);
            }
        }

        regions
    }
}

pub struct Region(HashSet<Coord>);

impl Region {
    fn new() -> Self {
        Self(HashSet::new())
    }

    fn contains(&self, coord: &Coord) -> bool {
        self.0.contains(coord)
    }

    fn insert(&mut self, coord: Coord) {
        self.0.insert(coord);
    }

    fn compute_area(&self) -> u32 {
        self.0.len() as u32
    }

    fn compute_perimeter(&self, map: &Map) -> u32 {
        // The perimeter of a region is the number of sides of garden plots in the region
        // that do not touch another garden plot in the same region
        let mut perimeter = 0;
        for coord in &self.0 {
            let mut count_touching_another_plot = 0;
            for neighbor in map.get_neighbors(coord) {
                if self.contains(&neighbor) {
                    count_touching_another_plot += 1;
                }
            }

            perimeter += 4 - count_touching_another_plot;
        }

        perimeter
    }

    fn neighbor_in_region(&self, map: &Map, coord: &Coord, dx: i32, dy: i32) -> bool {
        coord
            .step(dx, dy)
            .filter(|coord| map.is_in_bounds(coord))
            .map(|coord| self.contains(&coord))
            .unwrap_or_default()
    }

    fn all_neighbors(&self, map: &Map) -> HashSet<Coord> {
        self.0
            .iter()
            .flat_map(|coord| map.get_neighbors(coord))
            .collect::<HashSet<_>>()
            .difference(&self.0)
            .copied()
            .collect()
    }

    fn compute_number_of_sides(&self, map: &Map) -> u32 {
        // Under the bulk discount, instead of using the perimeter to calculate the price,
        // you need to use the number of sides each region has.
        // Each straight section of fence counts as a side, regardless of how long it is.

        // Number of sides equals number of corners
        let mut num_corners = 0;

        // First count all the convex corners.
        // A coord contains a convex corner if two adjacent neighbors are both not in the region.
        for coord in &self.0 {
            // check which neighbors are in the region
            let right = self.neighbor_in_region(map, coord, 1, 0);
            let left = self.neighbor_in_region(map, coord, -1, 0);
            let up = self.neighbor_in_region(map, coord, 0, 1);
            let down = self.neighbor_in_region(map, coord, 0, -1);

            // Check for two adjacent neighbors both not being in the region
            if !left && !up {
                num_corners += 1;
            }

            if !up && !right {
                num_corners += 1;
            }

            if !right && !down {
                num_corners += 1;
            }

            if !down && !left {
                num_corners += 1;
            }
        }

        // Then count all concave corners.
        // For each neighboring coord of the region, it is a concave corner if
        // two of its adjacent neighbors are both in the region
        // AND the diagonal neighbor in between those adjacent neighbors is also in the region.
        for coord in self.all_neighbors(map) {
            // check which neighbors are in the region
            let right = self.neighbor_in_region(map, &coord, 1, 0);
            let left = self.neighbor_in_region(map, &coord, -1, 0);
            let up = self.neighbor_in_region(map, &coord, 0, 1);
            let down = self.neighbor_in_region(map, &coord, 0, -1);

            // check diagonal neighbors
            let left_up = self.neighbor_in_region(map, &coord, -1, 1);
            let up_right = self.neighbor_in_region(map, &coord, 1, 1);
            let right_down = self.neighbor_in_region(map, &coord, 1, -1);
            let down_left = self.neighbor_in_region(map, &coord, -1, -1);

            if left && up && left_up {
                num_corners += 1;
            }

            if up && right && up_right {
                num_corners += 1;
            }

            if right && down && right_down {
                num_corners += 1;
            }

            if down && left && down_left {
                num_corners += 1;
            }
        }

        num_corners
    }
}

fn parse(input: &str) -> Map {
    let dimensions = get_grid_dimensions(input);
    let vals = input
        .lines()
        .rev()
        .map(|line| line.chars().collect())
        .collect();

    Map { dimensions, vals }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);

    let price = map
        .find_regions()
        .into_iter()
        .map(|region| region.compute_area() * region.compute_perimeter(&map))
        .sum();

    Some(price)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse(input);

    let price = map
        .find_regions()
        .into_iter()
        .map(|region| region.compute_area() * region.compute_number_of_sides(&map))
        .sum();

    Some(price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(368));
    }
}
