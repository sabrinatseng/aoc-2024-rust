use std::{collections::HashSet, fmt::Debug, str::FromStr};

pub mod template;

/// Parse input string into Vec of Vec of multiple items per line
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
    pub x: i64,
    pub y: i64,
}

impl Coord {
    pub fn new(x: i64, y: i64) -> Coord {
        Coord { x, y }
    }

    pub fn step(&self, dx: i64, dy: i64) -> Coord {
        Coord::new(self.x + dx, self.y + dy)
    }

    pub fn diff(&self, other: &Coord) -> (i64, i64) {
        (self.x - other.x, self.y - other.y)
    }
}

/// Dimensions of a 2D grid.
#[derive(Clone, Copy, Debug)]
pub struct Dimensions {
    pub x: usize,
    pub y: usize,
}

impl Dimensions {
    /// Assuming input is a 2-dimensional rectangular grid (i.e. all lines
    /// are the same length), return the dimensions of the grid.
    pub fn from_input(input: &str) -> Dimensions {
        let y_dim = input.lines().count();
        let x_dim = input.lines().next().unwrap().len();
        Dimensions { x: x_dim, y: y_dim }
    }

    pub fn in_bounds(&self, coord: &Coord) -> bool {
        coord.x >= 0 && (coord.x as usize) < self.x && coord.y >= 0 && (coord.y as usize) < self.y
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    pub dimensions: Dimensions,
    pub values: Vec<Vec<T>>,
}

impl<T: Clone> Grid<T> {
    pub fn new(dimensions: Dimensions, values: Vec<Vec<T>>) -> Self {
        assert_eq!(dimensions.x, values[0].len());
        assert_eq!(dimensions.y, values.len());
        Grid { dimensions, values }
    }

    pub fn in_bounds(&self, coord: &Coord) -> bool {
        self.dimensions.in_bounds(coord)
    }

    pub fn get(&self, coord: &Coord) -> Option<T> {
        if !self.in_bounds(coord) {
            return None;
        }

        self.values
            .get(coord.y as usize)?
            .get(coord.x as usize)
            .cloned()
    }

    pub fn get_neighbors<'a>(&'a self, coord: &'a Coord) -> impl Iterator<Item = Coord> + 'a {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(dx, dy)| coord.step(dx, dy))
            .filter(|coord| self.in_bounds(coord))
    }

    pub fn get_diagonal_neighbors<'a>(
        &'a self,
        coord: &'a Coord,
    ) -> impl Iterator<Item = Coord> + 'a {
        [(1, -1), (-1, 1), (1, 1), (-1, -1)]
            .into_iter()
            .map(|(dx, dy)| coord.step(dx, dy))
            .filter(|coord| self.in_bounds(coord))
    }

    pub fn positions_of(&self, val: &T) -> HashSet<Coord>
    where
        T: PartialEq,
    {
        let mut positions = HashSet::new();
        for x in 0..self.dimensions.x {
            for y in 0..self.dimensions.y {
                let coord = Coord::new(x as i64, y as i64);
                if self.get(&coord).as_ref() == Some(val) {
                    positions.insert(coord);
                }
            }
        }

        positions
    }
}
