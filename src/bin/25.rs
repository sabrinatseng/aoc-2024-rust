advent_of_code::solution!(25);

// Pin heights
#[derive(Copy, Clone)]
struct Lock([u8; 5]);

#[derive(Copy, Clone)]
struct Key([u8; 5]);

fn parse(input: &str) -> (Vec<Lock>, Vec<Key>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for schematic in input.split("\n\n") {
        let mut lines = schematic.trim().lines();
        let first_line = lines.next().unwrap();
        let lock = first_line.starts_with("#####");

        let mut pin_heights = if lock { [0; 5] } else { [5; 5] };

        for (i, line) in lines.enumerate() {
            if i >= 5 {
                // skip last line
                break;
            }

            for (j, c) in line.chars().enumerate() {
                if lock && c == '#' {
                    pin_heights[j] += 1;
                } else if !lock && c == '.' {
                    pin_heights[j] -= 1;
                }
            }
        }

        if lock {
            locks.push(Lock(pin_heights))
        } else {
            keys.push(Key(pin_heights))
        }
    }

    (locks, keys)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (locks, keys) = parse(input);

    let mut count = 0;
    for lock in locks {
        for key in keys.clone() {
            let mut fit = true;
            for i in 0..5 {
                if lock.0[i] + key.0[i] > 5 {
                    fit = false;
                    break;
                }
            }

            if fit {
                count += 1;
            }
        }
    }

    // Alternative functional solution
    // let count = locks
    //     .into_iter()
    //     .flat_map(|lock| keys.clone().into_iter().map(move |key| (lock, key)))
    //     .filter(|(lock, key)| (0..5).all(|i| lock.0[i] + key.0[i] <= 5))
    //     .count();

    Some(count as u32)
}

// No part two
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
