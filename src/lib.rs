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
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_dx_dy(self) -> (i64, i64) {
        match self {
            Self::Up => (0, 1),
            Self::Down => (0, -1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }

    pub fn turn_right(&self) -> Self {
        match &self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }

    pub fn turn_left(&self) -> Self {
        match &self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }
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

    pub fn step_in_direction(&self, direction: Direction) -> Coord {
        let (dx, dy) = direction.to_dx_dy();
        self.step(dx, dy)
    }

    pub fn diff(&self, other: &Coord) -> (i64, i64) {
        (self.x - other.x, self.y - other.y)
    }
}

impl From<(i64, i64)> for Coord {
    fn from((x, y): (i64, i64)) -> Self {
        Coord::new(x, y)
    }
}

/// Dimensions of a 2D grid.
#[derive(Clone, Copy, Debug)]
pub struct Dimensions {
    pub x: usize,
    pub y: usize,
}

impl Dimensions {
    pub fn new(x: usize, y: usize) -> Self {
        Dimensions { x, y }
    }

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

    /// If coord is out of bounds, wrap around to the other side
    pub fn wrap(&self, coord: &Coord) -> Coord {
        let new_x = if coord.x < 0 {
            let diff = (-coord.x) % self.x as i64;
            (self.x as i64 - diff) % self.x as i64
        } else if coord.x > self.x as i64 {
            (coord.x - self.x as i64) % self.x as i64
        } else {
            coord.x
        };

        let new_y = if coord.y < 0 {
            let diff = (-coord.y) % self.y as i64;
            (self.y as i64 - diff) % self.y as i64
        } else if coord.y > self.y as i64 {
            (coord.y - self.y as i64) % self.y as i64
        } else {
            coord.y
        };

        Coord::new(new_x, new_y)
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

    pub fn small_corner(&self) -> Coord {
        Coord::new(0, 0)
    }

    pub fn large_corner(&self) -> Coord {
        Coord::new(self.x as i64 - 1, self.y as i64 - 1)
    }

    pub fn left_borders(&self) -> impl Iterator<Item = Coord> + '_ {
        (0..self.y).map(|y| Coord::new(0, y as i64))
    }

    pub fn right_borders(&self) -> impl Iterator<Item = Coord> + '_ {
        (0..self.y).map(|y| Coord::new(self.x as i64 - 1, y as i64))
    }

    pub fn bottom_borders(&self) -> impl Iterator<Item = Coord> + '_ {
        (0..self.x).map(|x| Coord::new(x as i64, 0))
    }

    pub fn top_borders(&self) -> impl Iterator<Item = Coord> + '_ {
        (0..self.x).map(|x| Coord::new(x as i64, self.y as i64 - 1))
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
        self.dimensions.get_neighbors(coord)
    }

    pub fn get_diagonal_neighbors<'a>(
        &'a self,
        coord: &'a Coord,
    ) -> impl Iterator<Item = Coord> + 'a {
        self.dimensions.get_diagonal_neighbors(coord)
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

pub struct Maze {
    pub start: Coord,
    pub end: Coord,
    pub walls: HashSet<Coord>,
}

pub fn parse_maze(input: &str) -> Maze {
    let mut start = None;
    let mut end = None;
    let mut walls = HashSet::new();

    // Use coordinate system with (0,0) at bottom left
    for (y, line) in input.lines().rev().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = Coord::new(x as i64, y as i64);
            if c == 'S' {
                start = Some(coord);
            } else if c == 'E' {
                end = Some(coord);
            } else if c == '#' {
                walls.insert(coord);
            }
        }
    }

    Maze {
        start: start.expect("Did not find starting position S"),
        end: end.expect("Did not find end position E"),
        walls,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dim_wrap() {
        let dim = Dimensions::new(10, 10);

        // no wrap around
        assert_eq!(dim.wrap(&Coord::new(5, 5)), Coord::new(5, 5));

        // single wrap around for x
        assert_eq!(dim.wrap(&Coord::new(-2, 0)), Coord::new(8, 0));
        assert_eq!(dim.wrap(&Coord::new(12, 0)), Coord::new(2, 0));

        // single wrap around for y
        assert_eq!(dim.wrap(&Coord::new(0, -2)), Coord::new(0, 8));
        assert_eq!(dim.wrap(&Coord::new(0, 12)), Coord::new(0, 2));

        // single wrap around for both
        assert_eq!(dim.wrap(&Coord::new(-2, 12)), Coord::new(8, 2));

        // double wrap around
        assert_eq!(dim.wrap(&Coord::new(-12, 0)), Coord::new(8, 0));

        // multiple wrap around on the edges
        assert_eq!(dim.wrap(&Coord::new(0, -200)), Coord::new(0, 0));

        // regression test
        assert_eq!(
            Dimensions::new(11, 7).wrap(&Coord::new(4, -100)),
            Coord::new(4, 5)
        );
    }
}
