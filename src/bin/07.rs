use itertools::Itertools;

advent_of_code::solution!(7);

pub struct UnfinishedCalibrationEquation {
    result: u64,
    operands: Vec<u64>,
}

impl UnfinishedCalibrationEquation {
    fn could_be_true(&self, operators: &[fn(u64, u64) -> u64]) -> bool {
        // Base cases
        if self.operands.is_empty() {
            false
        } else if self.operands.len() == 1 {
            self.result == self.operands[0]
        } else if self.operands.len() == 2 {
            // Try each operator
            let x = self.operands[0];
            let y = self.operands[1];

            operators
                .iter()
                .any(|operator| operator(x, y) == self.result)
        } else {
            // Try each operator to the first 2 operands then construct a new
            // unfinished equation and recursively check
            let x = self.operands[0];
            let y = self.operands[1];

            operators.iter().any(|operator| {
                let mut new_operands = vec![operator(x, y)];
                new_operands.extend_from_slice(&self.operands[2..]);
                let new_eq = UnfinishedCalibrationEquation {
                    result: self.result,
                    operands: new_operands,
                };

                new_eq.could_be_true(operators)
            })
        }
    }
}

// Define operators

fn add(x: u64, y: u64) -> u64 {
    x + y
}

fn mult(x: u64, y: u64) -> u64 {
    x * y
}

fn concat(x: u64, y: u64) -> u64 {
    let y_len = y.to_string().len();
    x * (10_u64
        .checked_pow(y_len as u32)
        .unwrap_or_else(|| panic!("Overflow calculating 10^{y_len}")))
        + y
}

fn parse(input: &str) -> impl Iterator<Item = UnfinishedCalibrationEquation> + '_ {
    input.lines().map(|line| {
        let (result, operands) = line
            .split(':')
            .collect_tuple()
            .unwrap_or_else(|| panic!("Could not split line {line} by colon"));

        let result =
            str::parse(result).unwrap_or_else(|_| panic!("Failed to parse {result} into u32"));

        let operands = operands
            .split_whitespace()
            .map(|num| str::parse(num).unwrap_or_else(|_| panic!("Failed to parse {num} into u32")))
            .collect();

        UnfinishedCalibrationEquation { result, operands }
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total_calibration_result = 0;

    let operators = [add, mult];
    for equation in parse(input) {
        if equation.could_be_true(&operators) {
            total_calibration_result += equation.result;
        }
    }

    Some(total_calibration_result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total_calibration_result = 0;

    let operators = [add, mult, concat];
    for equation in parse(input) {
        if equation.could_be_true(&operators) {
            total_calibration_result += equation.result;
        }
    }

    Some(total_calibration_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
