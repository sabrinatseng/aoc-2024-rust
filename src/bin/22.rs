use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(22);

fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| {
            line.parse()
                .unwrap_or_else(|e| panic!("Failed to parse u32 from {line}: {e}"))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let secret_numbers = parse(input);

    let mut sum = 0;

    for mut secret_number in secret_numbers {
        for _ in 0..2000 {
            secret_number = next(secret_number);
        }

        sum += secret_number;
    }

    Some(sum)
}

// Get the next secret number
fn next(num: u64) -> u64 {
    // mix: secret number becomes secret number XOR val
    // prune: secret number becomes secret number mod 16777216 = 2^24

    // Multiply by 64 = 2^6, mix and prune
    let num = (num ^ (num << 6)) % 16777216;
    // Divide by 32 = 2^5, mix and prune
    let num = (num ^ (num >> 5)) % 16777216;
    // Multiply by 2048 = 2^11, mix and prune
    (num ^ (num << 11)) % 16777216
}

pub fn part_two(input: &str) -> Option<u32> {
    let secret_numbers = parse(input);

    // Brute force - go through all the secret number values
    // and store the number of bananas for each sequence of changes
    let mut bananas_for_sequence = HashMap::new();

    for mut secret_number in secret_numbers {
        let mut visited_sequences = HashSet::new();
        let mut diffs = Diffs::new();
        for _ in 0..2000 {
            let new_secret_number = next(secret_number);

            let old_price = secret_number % 10;
            let new_price = new_secret_number % 10;

            let diff = new_price as i8 - old_price as i8;

            diffs.push(diff);

            if let Some(sequence) = diffs.get() {
                // Only consider the first time a sequence is visited for each monkey
                if !visited_sequences.contains(&sequence) {
                    *bananas_for_sequence.entry(sequence).or_default() += new_price as u32;

                    visited_sequences.insert(sequence);
                }
            }

            secret_number = new_secret_number;
        }
    }

    bananas_for_sequence.into_values().max()
}

// A sequence of 4 price changes
// Treat as a ring buffer to simplify push & shift operations
struct Diffs {
    diffs: [i8; 4],
    ptr: usize,
}

impl Diffs {
    fn new() -> Self {
        Diffs {
            diffs: [0; 4],
            ptr: 0,
        }
    }

    fn get(&self) -> Option<(i8, i8, i8, i8)> {
        if self.ptr < 4 {
            // We haven't seen 4 price changes yet
            return None;
        }

        // Start at ptr % 4 and wrap around
        let i = self.ptr % 4;
        let tup = self.diffs[i..]
            .iter()
            .chain(self.diffs[..i].iter())
            .copied()
            .collect_tuple()
            .expect("Did not get 4 diffs");

        Some(tup)
    }

    fn push(&mut self, diff: i8) {
        self.diffs[self.ptr % 4] = diff;
        self.ptr += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        // Example given in the problem
        let secret_number_iterations = vec![
            123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484,
            7753432, 5908254,
        ];

        for i in 1..secret_number_iterations.len() {
            assert_eq!(
                next(secret_number_iterations[i - 1]),
                secret_number_iterations[i]
            );
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
