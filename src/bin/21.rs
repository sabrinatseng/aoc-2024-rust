use cached::proc_macro::cached;
use lazy_static::lazy_static;
use std::{cmp::Ordering, collections::HashMap, hash::Hash};

use advent_of_code::{Coord, Direction};

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u32> {
    let codes = input.lines().collect::<Vec<_>>();

    let mut sum = 0;
    for code in codes {
        // numeric keypad
        let path1 = find_shortest_path_for_sequence(&NUMERIC_KEYPAD, code);
        // directional keypad 1
        let path2 = find_shortest_path_for_sequence(&DIRECTIONAL_KEYPAD, &path1);
        // directional keypad 2
        let path3 = find_shortest_path_for_sequence(&DIRECTIONAL_KEYPAD, &path2);

        let shortest_sequence_len = path3.len() as u32;
        let numeric_part_of_code: u32 = code
            .split_at(code.len() - 1)
            .0
            .parse()
            .expect("Failed to parse numeric part of code");

        sum += shortest_sequence_len * numeric_part_of_code;
    }

    Some(sum)
}

struct Keypad {
    button_mapping: HashMap<char, Coord>,
    reverse_button_mapping: HashMap<Coord, char>,
    // y-coordinate of the gap
    gap_y: i64,
}

impl Keypad {
    fn new(button_mapping: HashMap<char, Coord>, gap_y: i64) -> Self {
        let reverse_button_mapping = button_mapping.iter().map(|(c, loc)| (*loc, *c)).collect();
        Self {
            button_mapping,
            reverse_button_mapping,
            gap_y,
        }
    }
}

impl Hash for Keypad {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.gap_y.hash(state);
    }
}

impl PartialEq for Keypad {
    fn eq(&self, other: &Self) -> bool {
        self.gap_y == other.gap_y
    }
}

impl Eq for Keypad {}

lazy_static! {
    // (0, 0) is the gap at the bottom left
    static ref NUMERIC_KEYPAD: Keypad = Keypad::new(
        HashMap::from([
            ('0', Coord::new(1, 0)),
            ('A', Coord::new(2, 0)),
            ('1', Coord::new(0, 1)),
            ('2', Coord::new(1, 1)),
            ('3', Coord::new(2, 1)),
            ('4', Coord::new(0, 2)),
            ('5', Coord::new(1, 2)),
            ('6', Coord::new(2, 2)),
            ('7', Coord::new(0, 3)),
            ('8', Coord::new(1, 3)),
            ('9', Coord::new(2, 3)),
        ]),
        0,
    );

    static ref DIRECTIONAL_KEYPAD: Keypad = Keypad::new(
        HashMap::from([
        ('<', Coord::new(0, 0)),
        ('v', Coord::new(1, 0)),
        ('>', Coord::new(2, 0)),
        ('^', Coord::new(1, 1)),
        ('A', Coord::new(2, 1)),
    ]),
    1,
    );

    static ref DIRECTIONS: HashMap<char, Direction> = HashMap::from([
        ('<', Direction::Left),
        ('>', Direction::Right),
        ('^', Direction::Up),
        ('v', Direction::Down),
    ]);
}

// Return shortest sequence to type out a sequence on the given keypad using a directional keypad
fn find_shortest_path_for_sequence(keypad: &'static Keypad, sequence: &str) -> String {
    if sequence.is_empty() {
        return "".to_string();
    }

    let mut curr = 'A'; // always start at the A

    let mut output = String::new();
    for c in sequence.chars() {
        // navigate to the next button
        let path = find_shortest_path(keypad, curr, c);
        output.push_str(&path);

        curr = c;
    }

    output
}

// Shortest path from start to pressing end
// gap_y is the y-coordinate of the gap
#[cached]
fn find_shortest_path(keypad: &'static Keypad, start: char, end: char) -> String {
    // add the button press 'A'
    format!("{}A", find_shortest_path_inner(keypad, start, end))
}

// Find shortest path from start to end (excluding the button press)
fn find_shortest_path_inner(keypad: &'static Keypad, start: char, end: char) -> String {
    if start == end {
        // we are already in the right position
        return "".to_string();
    }

    let start = keypad.button_mapping.get(&start).unwrap();
    let end = keypad.button_mapping.get(&end).unwrap();
    let (dx, dy) = end.diff(start);

    // There are 2 segments to the path - the horizontal (dx) and vertical (dy)
    let dx_path = dx_to_path(dx);
    let dy_path = dy_to_path(dy);

    // If we only need to move in one direction, there is only one shortest path
    if dx == 0 {
        return dy_path;
    } else if dy == 0 {
        return dx_path;
    }

    // If one path crosses over the gap, that is not allowed so only the other path is possible
    // This happens when going from the left wall to the bottom wall or vice versa
    // or left to top or vice versa for the directional keypad
    if start.x == 0 && end.y == keypad.gap_y {
        // going from left wall to top/bottom wall - must go right then up/down
        return format!("{dx_path}{dy_path}");
    } else if start.y == keypad.gap_y && end.x == 0 {
        // going from top/bottom wall to left wall - must go down/up then left
        return format!("{dy_path}{dx_path}");
    }

    // In other cases, we should always prefer going left first, then up/down, then right.
    // This is based on distance from the button A - going to the furthest button first
    // shortens the overall distance since we always have to end with A.
    if dx < 0 {
        format!("{dx_path}{dy_path}")
    } else {
        format!("{dy_path}{dx_path}")
    }
}

fn dx_to_path(dx: i64) -> String {
    match dx.cmp(&0) {
        Ordering::Equal => "".to_string(),
        Ordering::Greater => ">".repeat(dx as usize),
        Ordering::Less => "<".repeat(-dx as usize),
    }
}

fn dy_to_path(dy: i64) -> String {
    match dy.cmp(&0) {
        Ordering::Equal => "".to_string(),
        Ordering::Greater => "^".repeat(dy as usize),
        Ordering::Less => "v".repeat(-dy as usize),
    }
}

// for debugging - translate a sequence of button presses on a directional keypad, into
// what buttons would be pressed on the given keypad
#[allow(dead_code)]
fn translate(sequence: &str, keypad: &Keypad) -> String {
    let start = keypad.button_mapping.get(&'A').unwrap();
    let mut curr = *start;
    let mut output = String::new();

    for c in sequence.chars() {
        if c == 'A' {
            output.push(*keypad.reverse_button_mapping.get(&curr).unwrap());
        } else {
            curr = curr.step_in_direction(*DIRECTIONS.get(&c).unwrap());
        }
    }

    output
}

// Directional keypad robots only
#[cached]
fn shortest_path_len_for_sequence_with_n_robots(n: usize, sequence: String) -> u64 {
    // base case
    if n == 0 {
        return sequence.len() as u64;
    }

    let mut path_len = 0;
    let mut curr = 'A';

    for c in sequence.chars() {
        path_len += shortest_path_len_for_sequence_with_n_robots(
            n - 1,
            find_shortest_path(&DIRECTIONAL_KEYPAD, curr, c),
        );
        curr = c;
    }

    path_len
}

pub fn part_two(input: &str) -> Option<u64> {
    let codes = input.lines().collect::<Vec<_>>();

    let mut sum = 0;

    for code in codes {
        // numeric keypad
        let path1 = find_shortest_path_for_sequence(&NUMERIC_KEYPAD, code);

        let shortest_sequence_len = shortest_path_len_for_sequence_with_n_robots(25, path1);
        let numeric_part_of_code: u64 = code
            .split_at(code.len() - 1)
            .0
            .parse()
            .expect("Failed to parse numeric part of code");

        sum += shortest_sequence_len * numeric_part_of_code;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_shortest_paths_numeric() {
        assert_eq!(&find_shortest_path(&NUMERIC_KEYPAD, 'A', '0'), "<A");
        assert_eq!(&find_shortest_path(&NUMERIC_KEYPAD, '0', '2'), "^A");
        assert_eq!(&find_shortest_path(&NUMERIC_KEYPAD, '2', '9'), "^^>A");
        assert_eq!(&find_shortest_path(&NUMERIC_KEYPAD, '9', 'A'), "vvvA");
    }

    #[test]
    fn test_find_shortest_sequence_numeric() {
        let sequence = find_shortest_path_for_sequence(&NUMERIC_KEYPAD, "029A");
        assert_eq!(sequence, "<A^A^^>AvvvA".to_string());
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // https://www.reddit.com/r/adventofcode/comments/1hjb7hh/2024_day_21_part_2_can_someone_share_what_the/
        assert_eq!(result, Some(154115708116294));
    }
}
