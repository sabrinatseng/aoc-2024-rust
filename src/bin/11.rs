use advent_of_code::parse_from_lines;
use cached::proc_macro::cached;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    // only 1 line of numbers
    let mut nums = parse_from_lines(input)
        .next()
        .unwrap()
        .collect::<Vec<u64>>();

    for _ in 0..25 {
        // Initialize with double the size to avoid reallocations
        let mut new_nums = Vec::with_capacity(nums.len() * 2);

        // Apply rules
        for i in nums {
            let num_digits = i.to_string().len();
            if i == 0 {
                // If the stone is engraved with the number 0,
                // it is replaced by a stone engraved with the number 1.
                new_nums.push(1);
            } else if num_digits % 2 == 0 {
                // If the stone is engraved with a number that has an even number of digits,
                // it is replaced by two stones.
                // The left half of the digits are engraved on the new left stone,
                // and the right half of the digits are engraved on the new right stone.
                // (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)

                let tens_power = 10_u64.pow(num_digits as u32 / 2);
                let left_half = i / tens_power;
                let right_half = i % tens_power;

                new_nums.push(left_half);
                new_nums.push(right_half);
            } else {
                // If none of the other rules apply, the stone is replaced by a new stone;
                // the old stone's number multiplied by 2024 is engraved on the new stone.
                new_nums.push(i * 2024);
            }
        }

        nums = new_nums;
    }

    Some(nums.len() as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let nums = parse_from_lines(input).next().unwrap();

    let sum = nums.map(|i| num_stones_after_n_blinks(i, 75)).sum::<u64>();

    Some(sum as u64)
}

/// Recursive and memoized computation of the number of stones resulting
/// from a single starting stone value and a number of blinks.
#[cached]
fn num_stones_after_n_blinks(initial: u64, blinks: usize) -> u64 {
    if blinks == 0 {
        // Base case -> just the one stone
        return 1;
    }

    if initial == 0 {
        // If the stone is engraved with the number 0,
        // it is replaced by a stone engraved with the number 1.
        return num_stones_after_n_blinks(1, blinks - 1);
    }

    let num_digits = initial.to_string().len();
    if num_digits % 2 == 0 {
        // If the stone is engraved with a number that has an even number of digits,
        // it is replaced by two stones.
        // The left half of the digits are engraved on the new left stone,
        // and the right half of the digits are engraved on the new right stone.
        // (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)

        let tens_power = 10_u64.pow(num_digits as u32 / 2);
        let left_half = initial / tens_power;
        let right_half = initial % tens_power;

        return num_stones_after_n_blinks(left_half, blinks - 1)
            + num_stones_after_n_blinks(right_half, blinks - 1);
    }

    // If none of the other rules apply, the stone is replaced by a new stone;
    // the old stone's number multiplied by 2024 is engraved on the new stone.
    num_stones_after_n_blinks(initial * 2024, blinks - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
