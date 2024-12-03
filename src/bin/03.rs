use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let sum = regex
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| parse_and_multiply(a, b))
        .sum();

    Some(sum)
}

fn parse_and_multiply(a: &str, b: &str) -> u32 {
    parse_match(a) * parse_match(b)
}

fn parse_match(match_: &str) -> u32 {
    match_
        .parse()
        .expect(&format!("Failed to parse regex match {match_} into u32"))
}

pub fn part_two(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut enabled = true;
    let mut sum = 0;
    for captures in regex.captures_iter(input) {
        if captures[0].eq("do()") {
            enabled = true;
        } else if captures[0].eq("don't()") {
            enabled = false;
        } else {
            // mul instruction
            if enabled {
                sum += parse_and_multiply(&captures[1], &captures[2]);
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
