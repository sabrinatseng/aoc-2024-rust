use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let word_search = WordSearch::new(input);

    let xs = word_search.get_locations_of('X');

    let mut count = 0;
    for (r, c) in xs.iter() {
        let r = *r as i32;
        let c = *c as i32;

        for (dr, dc) in [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ] {
            if word_search.is_xmas(r, c, dr, dc) {
                count += 1;
            }
        }
    }

    Some(count)
}

struct WordSearch {
    rows: usize,
    cols: usize,
    values: Vec<String>,
}

impl WordSearch {
    fn new(input: &str) -> Self {
        let values: Vec<String> = input.lines().map(|line| line.to_string()).collect();

        let rows = values.len();
        let cols = values[0].len();

        Self { rows, cols, values }
    }

    fn get_char_at(&self, r: i32, c: i32) -> Option<char> {
        if r < 0 || r >= self.rows as i32 || c < 0 || c >= self.cols as i32 {
            return None;
        }

        self.values
            .get(r as usize)
            .and_then(|line| line.chars().nth(c as usize))
    }

    fn get_locations_of(&self, ch: char) -> HashSet<(usize, usize)> {
        let mut set = HashSet::new();
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.get_char_at(r as i32, c as i32) == Some(ch) {
                    set.insert((r, c));
                }
            }
        }

        set
    }

    fn is_xmas(&self, r: i32, c: i32, dr: i32, dc: i32) -> bool {
        // Assume we are starting from an 'X' already so skip the check
        self.get_char_at(r + dr, c + dc) == Some('M')
            && self.get_char_at(r + dr * 2, c + dc * 2) == Some('A')
            && self.get_char_at(r + dr * 3, c + dc * 3) == Some('S')
    }

    fn is_x_mas(&self, r: i32, c: i32) -> bool {
        // Assume we are starting from an 'A'
        let is_left_mas = (self.get_char_at(r - 1, c - 1) == Some('M')
            && self.get_char_at(r + 1, c + 1) == Some('S'))
            || (self.get_char_at(r - 1, c - 1) == Some('S')
                && self.get_char_at(r + 1, c + 1) == Some('M'));

        let is_right_mas = (self.get_char_at(r - 1, c + 1) == Some('M')
            && self.get_char_at(r + 1, c - 1) == Some('S'))
            || (self.get_char_at(r - 1, c + 1) == Some('S')
                && self.get_char_at(r + 1, c - 1) == Some('M'));

        is_left_mas && is_right_mas
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let word_search = WordSearch::new(input);

    let count = word_search
        .get_locations_of('A')
        .into_iter()
        .map(|(r, c)| word_search.is_x_mas(r as i32, c as i32))
        .filter(|b| *b)
        .count();

    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
