use std::collections::HashSet;

use advent_of_code::{Coord, Dimensions};
use itertools::Itertools;

advent_of_code::solution!(14);

struct Robot {
    pos: Coord,
    vel: Coord,
}

impl Robot {
    fn step_n(&mut self, dimensions: &Dimensions, n: usize) {
        let (dx, dy) = (self.vel.x * n as i64, self.vel.y * n as i64);

        let new_pos = self.pos.step(dx, dy);
        let wrapped = dimensions.wrap(&new_pos);

        self.pos = wrapped;
    }

    // Return quadrant number where the quadrants are numbered like this
    // (starting from top left and going clockwise):
    // ..... .....
    // ..0.. ..1..
    // ..... .....
    //
    // ..... .....
    // ..3.. ..2..
    // ..... .....
    //
    // Note how we number the quadrants is not important as long as they are consistent
    fn quadrant(&self, dimensions: &Dimensions) -> Option<usize> {
        let mid_x = (dimensions.x / 2) as i64;
        let mid_y = (dimensions.y / 2) as i64;

        if self.pos.x < mid_x && self.pos.y < mid_y {
            Some(0)
        } else if self.pos.x > mid_x && self.pos.y < mid_y {
            Some(1)
        } else if self.pos.x > mid_x && self.pos.y > mid_y {
            Some(2)
        } else if self.pos.x < mid_x && self.pos.y > mid_y {
            Some(3)
        } else {
            None
        }
    }
}

fn parse(input: &str) -> Vec<Robot> {
    input.lines().map(parse_robot).collect()
}

fn parse_robot(input: &str) -> Robot {
    let (p, v) = input
        .split(" ")
        .collect_tuple()
        .unwrap_or_else(|| panic!("{input} has more than one space"));

    let p_comma = p
        .find(',')
        .unwrap_or_else(|| panic!("{p} does not contain ,"));
    let p_x = p[2..p_comma]
        .parse()
        .unwrap_or_else(|e| panic!("Failed to parse x from {p}: {e}"));
    let p_y = p[(p_comma + 1)..]
        .parse()
        .unwrap_or_else(|e| panic!("Failed to parse y from {p}: {e}"));

    let v_comma = v
        .find(',')
        .unwrap_or_else(|| panic!("{v} does not contain ,"));
    let v_x = v[2..v_comma]
        .parse()
        .unwrap_or_else(|e| panic!("Failed to parse x from {v}: {e}"));
    let v_y = v[(v_comma + 1)..]
        .parse()
        .unwrap_or_else(|e| panic!("Failed to parse y from {v}: {e}"));

    Robot {
        pos: Coord::new(p_x, p_y),
        vel: Coord::new(v_x, v_y),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut robots = parse(input);

    // The example has different dimensions
    #[cfg(test)]
    let dimensions = Dimensions::new(11, 7);
    #[cfg(not(test))]
    let dimensions = Dimensions::new(101, 103);

    let mut quadrant_counts = [0, 0, 0, 0];

    for robot in robots.iter_mut() {
        robot.step_n(&dimensions, 100);

        if let Some(quadrant) = robot.quadrant(&dimensions) {
            quadrant_counts[quadrant] += 1;
        }
    }

    let safety_factor =
        quadrant_counts[0] * quadrant_counts[1] * quadrant_counts[2] * quadrant_counts[3];
    Some(safety_factor as u32)
}

// Return the max number of consecutive robots horizontally
fn consecutive_robots(robots: &[Robot], dimensions: &Dimensions) -> usize {
    let mut consecutive = 0;
    let mut max_consecutive = 0;
    let locs = robots.iter().map(|robot| robot.pos).collect::<HashSet<_>>();
    for x in 0..dimensions.x {
        for y in 0..dimensions.y {
            if locs.contains(&Coord::new(x as i64, y as i64)) {
                consecutive += 1;
            } else {
                if consecutive > max_consecutive {
                    max_consecutive = consecutive;
                }

                consecutive = 0;
            }
        }
    }

    max_consecutive
}

fn print_robots(robots: &[Robot], dimensions: &Dimensions, iterations: usize) {
    println!("Iteration {iterations}");

    let locs = robots.iter().map(|robot| robot.pos).collect::<HashSet<_>>();
    for x in 0..dimensions.x {
        for y in 0..dimensions.y {
            if locs.contains(&Coord::new(x as i64, y as i64)) {
                print!("+");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots = parse(input);

    // The example has different dimensions
    #[cfg(test)]
    let dimensions = Dimensions::new(11, 7);
    #[cfg(not(test))]
    let dimensions = Dimensions::new(101, 103);

    for i in 1..10000 {
        for robot in robots.iter_mut() {
            robot.step_n(&dimensions, 1);
        }

        // Look for a lot of consecutive robots to try to find the Christmas tree pattern
        if consecutive_robots(&robots, &dimensions) > 10 {
            print_robots(&robots, &dimensions, i);
            return Some(i as u32);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
