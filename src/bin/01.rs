use advent_of_code::parse_from_lines;
use itertools::Itertools;

advent_of_code::solution!(1);

// Parse input into left list and right list of equal length
fn parse_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    parse_from_lines::<u32>(input)
        .map(|iter| iter.collect_tuple().expect("Line does not have 2 numbers"))
        .unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut list_1, mut list_2) = parse_lists(input);

    list_1.sort();
    list_2.sort();

    let sum = list_1
        .into_iter()
        .zip(list_2)
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (list_1, list_2) = parse_lists(input);

    let list_2_freq = list_2.into_iter().counts();

    let similarity = list_1
        .into_iter()
        .map(|i| i * list_2_freq.get(&i).cloned().unwrap_or(0) as u32)
        .sum();

    Some(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
