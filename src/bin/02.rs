use advent_of_code::parse_from_lines;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let safe_reports = parse_from_lines::<u32>(input)
        .map(|report| is_safe(report.collect()))
        .filter(|b| *b)
        .count();

    Some(safe_reports as u32)
}

// Compute if a report is safe, i.e. if:
// - The levels are either all increasing or all decreasing.
// - Any two adjacent levels differ by at least one and at most three.
fn is_safe(report: Vec<u32>) -> bool {
    // This should never panic since we are coercing windows of size 2 into [u32; 2]
    let mut windows = report.windows(2).map(|slice| slice.try_into().unwrap());
    windows.clone().all(is_gradually_increasing) || windows.all(is_gradually_decreasing)
}

// Compute if two numbers are gradually increasing
fn is_gradually_increasing(window: &[u32; 2]) -> bool {
    window[1] > window[0] && (1..=3).contains(&window[1].abs_diff(window[0]))
}

// Compute if two numbers are gradually decreasing
fn is_gradually_decreasing(window: &[u32; 2]) -> bool {
    window[1] < window[0] && (1..=3).contains(&window[1].abs_diff(window[0]))
}

pub fn part_two(input: &str) -> Option<u32> {
    let safe_reports = parse_from_lines::<u32>(input)
        .map(|report| is_safe_with_dampener(report.collect()))
        .filter(|b| *b)
        .count();

    Some(safe_reports as u32)
}

// Compute if a report is safe if we remove at most 1 element
fn is_safe_with_dampener(report: Vec<u32>) -> bool {
    // Try removing each element
    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);

        if is_safe(modified_report) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
