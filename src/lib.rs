use std::{fmt::Debug, str::FromStr};

pub mod template;

// Parse input string into Vec of Vec of multiple items per line
pub fn parse_from_lines<'a, T>(
    input: &'a str,
) -> impl Iterator<Item = impl Iterator<Item = T> + 'a> + 'a
where
    T: FromStr + 'a,
    T::Err: Debug,
{
    input.lines().map(|line| {
        line.split_whitespace()
            .map(str::parse::<T>)
            .map(|res| res.expect("Failed to parse"))
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }

    pub fn step(&self, dx: i32, dy: i32) -> Option<Coord> {
        let new_x = self.x as i32 + dx;
        let new_y = self.y as i32 + dy;
        if new_x < 0 || new_y < 0 {
            return None;
        }

        Some(Coord::new(new_x as usize, new_y as usize))
    }

    pub fn diff(&self, other: &Coord) -> (i32, i32) {
        (
            self.x as i32 - other.x as i32,
            self.y as i32 - other.y as i32,
        )
    }
}
