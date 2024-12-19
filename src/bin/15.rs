use std::collections::{HashSet, VecDeque};

use advent_of_code::{Coord, Dimensions, Direction};
use itertools::Itertools;

advent_of_code::solution!(15);

struct Grid {
    dimensions: Dimensions,
    walls: HashSet<Coord>,
    boxes: HashSet<Coord>,
    robot: Coord,
}

impl Grid {
    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.dimensions.y {
            for x in 0..self.dimensions.x {
                let coord = Coord::new(x as i64, y as i64);
                if self.robot == coord {
                    print!("@");
                } else if self.boxes.contains(&coord) {
                    print!("O");
                } else if self.walls.contains(&coord) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn expand_for_part_two(self) -> Self {
        let Grid {
            dimensions,
            walls,
            boxes,
            robot,
        } = self;

        let new_dimensions = Dimensions::new(dimensions.x * 2, dimensions.y);
        let new_walls = walls
            .into_iter()
            .flat_map(|coord| {
                let Coord { x, y } = coord;
                [Coord::new(x * 2, y), Coord::new(x * 2 + 1, y)]
            })
            .collect();
        // Store just the left half of each box
        let new_boxes = boxes
            .into_iter()
            .map(|coord| Coord::new(coord.x * 2, coord.y))
            .collect();
        let new_robot = Coord::new(robot.x * 2, robot.y);

        Self {
            dimensions: new_dimensions,
            walls: new_walls,
            boxes: new_boxes,
            robot: new_robot,
        }
    }

    // For wide boxes in part 2
    fn is_left_half_of_box(&self, coord: &Coord) -> bool {
        self.boxes.contains(coord)
    }

    // For wide boxes in part 2
    fn is_right_half_of_box(&self, coord: &Coord) -> bool {
        self.boxes.contains(&Coord::new(coord.x - 1, coord.y))
    }

    // For wide boxes in part 2
    // Assume coord is either a left or right half
    fn get_both_halves_of_box(&self, coord: &Coord) -> (Coord, Coord) {
        if self.is_left_half_of_box(coord) {
            (*coord, Coord::new(coord.x + 1, coord.y))
        } else {
            assert!(self.is_right_half_of_box(coord));
            (Coord::new(coord.x - 1, coord.y), *coord)
        }
    }

    #[allow(dead_code)]
    fn print_for_part_two(&self) {
        for y in 0..self.dimensions.y {
            for x in 0..self.dimensions.x {
                let coord = Coord::new(x as i64, y as i64);
                if self.robot == coord {
                    print!("@");
                } else if self.is_left_half_of_box(&coord) {
                    print!("[");
                } else if self.is_right_half_of_box(&coord) {
                    print!("]");
                } else if self.walls.contains(&coord) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn parse_map(input: &str) -> Grid {
    let dimensions = Dimensions::from_input(input);
    let mut robot = None;
    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();

    // Use coordinate system with (0, 0) at the top left
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '@' => {
                    robot = Some(Coord::new(x as i64, y as i64));
                }
                '#' => {
                    walls.insert(Coord::new(x as i64, y as i64));
                }
                'O' => {
                    boxes.insert(Coord::new(x as i64, y as i64));
                }
                '.' => {}
                c => panic!("Unexpected character {c} in map"),
            }
        }
    }

    Grid {
        dimensions,
        walls,
        boxes,
        robot: robot.expect("Did not find robot character @"),
    }
}

fn parse_movements(input: &str) -> Vec<Direction> {
    input
        .lines()
        .flat_map(|line| line.chars().map(parse_movement))
        .collect()
}

fn parse_movement(c: char) -> Direction {
    match c {
        // Because our coordinate system uses (0, 0) as the top left,
        // ^ actually means to decrease the y-coordinate while v means to increase it,
        // which is the opposite of the convention so we swap the directions here
        '^' => Direction::Down,
        'v' => Direction::Up,
        '<' => Direction::Left,
        '>' => Direction::Right,
        c => panic!("Unexpected char {c} could not be parsed as movement"),
    }
}

fn parse(input: &str) -> (Grid, Vec<Direction>) {
    let (map, movements) = input
        .split("\n\n")
        .collect_tuple()
        .expect("Expected two blocks separated by newline");

    (parse_map(map), parse_movements(movements))
}

fn compute_gps_coordinate(coord: &Coord) -> u32 {
    100 * coord.y as u32 + coord.x as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut map, movements) = parse(input);

    for dir in movements {
        let new_robot_coord = map.robot.step_in_direction(dir);

        // If there is a box, keep going until we reach the end of the chain
        let mut coord = new_robot_coord;
        let mut boxes = 0;
        while map.boxes.contains(&coord) {
            coord = coord.step_in_direction(dir);
            boxes += 1;
        }

        // At the end of the line of boxes:
        if !map.walls.contains(&coord) {
            // If there is an empty space, push all the boxes. The result is
            // that the robot moves over, the first box is removed and the last box
            // is added.
            if boxes > 0 {
                // If there were boxes, push the boxes
                map.boxes.insert(coord);
                map.boxes.remove(&new_robot_coord);
            }

            // Move the robot
            map.robot = new_robot_coord;
        } // Otherwise we've hit a wall so don't move anything

        #[cfg(test)]
        map.print();
    }

    let sum = map.boxes.iter().map(compute_gps_coordinate).sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, movements) = parse(input);
    let mut map = map.expand_for_part_two();

    for dir in movements {
        let new_robot_coord = map.robot.step_in_direction(dir);

        // If there are boxes, keep going until we reach the end of the chain
        // Keep a queue of boxes to be pushed since both left and right halves might need to be checked
        let mut queue = VecDeque::new();
        queue.push_back(new_robot_coord);

        // Keep track of which boxes should be pushed
        let mut boxes_to_push = HashSet::new();

        // If any box hits a wall, the entire set of boxes does not move
        let mut hit_wall = false;
        while let Some(coord) = queue.pop_front() {
            if map.is_left_half_of_box(&coord) || map.is_right_half_of_box(&coord) {
                let (left_half, right_half) = map.get_both_halves_of_box(&coord);

                // make sure to insert the left half of box to be consistent with our representation
                boxes_to_push.insert(left_half);

                // If moving right/left, we only need to check the furthest half of the box
                // If we're moving up/down, add both halves of the box onto the queue
                match dir {
                    Direction::Up | Direction::Down => {
                        queue.push_back(left_half.step_in_direction(dir));
                        queue.push_back(right_half.step_in_direction(dir));
                    }
                    Direction::Left => {
                        queue.push_back(left_half.step_in_direction(dir));
                    }
                    Direction::Right => {
                        queue.push_back(right_half.step_in_direction(dir));
                    }
                }
            } else if map.walls.contains(&coord) {
                hit_wall = true;
                break;
            }
        }

        // At the end of the line of boxes:
        if !hit_wall {
            // If there is an empty space, push all the boxes and move the robot.
            if !boxes_to_push.is_empty() {
                // If there were boxes, push the boxes
                // Remove them all and then add them all back in their new locations
                // to avoid overwriting anything
                for box_ in boxes_to_push.iter() {
                    map.boxes.remove(box_);
                }

                for box_ in boxes_to_push {
                    map.boxes.insert(box_.step_in_direction(dir));
                }
            }

            // Move the robot
            map.robot = new_robot_coord;
        } // Otherwise we've hit a wall so don't move anything

        #[cfg(test)]
        map.print_for_part_two();
    }

    let sum = map.boxes.iter().map(compute_gps_coordinate).sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_small() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one_large() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two_large() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9021));
    }

    #[test]
    fn test_part_two_small() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        let expected = 105 + 207 + 306;
        assert_eq!(result, Some(expected));
    }
}
