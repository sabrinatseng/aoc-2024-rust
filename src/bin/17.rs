use std::{collections::VecDeque, fmt::Display};

use itertools::Itertools;

advent_of_code::solution!(17);

fn parse(input: &str) -> ([u64; 3], Vec<u8>) {
    let (registers, program) = input
        .split("\n\n")
        .collect_tuple()
        .expect("Expected two blocks in input");

    (parse_registers(registers), parse_program(program))
}

fn parse_program(input: &str) -> Vec<u8> {
    let input = &input.trim()["Program: ".len()..];
    input
        .split(",")
        .map(|s| str::parse(s).unwrap_or_else(|e| panic!("Failed to parse int from {s}: {e}")))
        .collect()
}

fn parse_registers(input: &str) -> [u64; 3] {
    input
        .trim()
        .lines()
        .map(parse_register)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap_or_else(|_| panic!("Did not find 3 registers"))
}

fn parse_register(input: &str) -> u64 {
    input["Register A: ".len()..]
        .parse()
        .unwrap_or_else(|_| panic!("Could not parse register {input}"))
}

fn combo_operand(operand: u8, registers: &[u64; 3]) -> u64 {
    if operand <= 3 {
        operand as u64
    } else {
        registers[operand as usize - 4]
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut registers, program) = parse(input);

    let output = run_program(&mut registers, program);

    Some(output.into_iter().join(","))
}

fn run_program(registers: &mut [u64; 3], program: Vec<u8>) -> Vec<u8> {
    // Instruction pointer
    let mut ip = 0;

    let mut output = Vec::new();

    while ip < program.len() - 1 {
        // These are safe to unwrap since we check the bounds in the while condition
        let opcode = *program.get(ip).unwrap();
        let operand = *program.get(ip + 1).unwrap();

        match opcode {
            0 => {
                // adv (division)
                registers[0] >>= combo_operand(operand, registers);
            }
            1 => {
                // bxl (bitwise xor)
                let left = registers[1];
                let right = operand;
                registers[1] = left ^ right as u64;
            }
            2 => {
                // bst (mod 8)
                let left = combo_operand(operand, registers);
                registers[1] = left % 8;
            }
            3 => {
                // jnz (jump)
                if registers[0] != 0 {
                    ip = operand as usize;
                    continue; // don't increment the ip
                }
            }
            4 => {
                // bxc (bitwise xor)
                let left = registers[1];
                let right = registers[2];
                registers[1] = left ^ right;
            }
            5 => {
                // out (combo operand mod 8)
                let left = combo_operand(operand, registers);
                output.push((left % 8) as u8);
            }
            6 => {
                // bdv (division)
                registers[1] = registers[0] >> combo_operand(operand, registers);
            }
            7 => {
                // cdv (division)
                registers[2] = registers[0] >> combo_operand(operand, registers);
            }
            opcode => panic!("Invalid opcode {opcode}"),
        }

        ip += 2;
    }

    output
}

pub fn part_two(input: &str) -> Option<u64> {
    // The program is as follows:
    // 2,4: B = A mod 8 (lowest 3 bits of A)                    <- call this a_mod_8
    // 1,6: B = B ^ 6 (110)                                     <- call this constrained_bits_idx
    // 7,5: C = A // (2^B) (A shifted to the right by B bits)   <- call lowest 3 bits of C constrained_bits
    // 4,6: B = B ^ C
    // 1,4: B = B ^ 4 (100)
    // 5,5: output B % 8 (lowest 3 bits of B)                   <- call the output val
    // 0,3: A = A // (2^3) (shift A to the right 3 bits)
    // 3,0: jump back to the start of the program

    // The value that gets output is (lowest 3 bits of A) ^ 6 ^ (some higher 3 bits of A) ^ 4.
    // The xor with 6 (110) and 4 (100) can be combined into (6 ^ 4) = 2 (010).

    // The output val is some number 0..8 defined by the program.
    // val = (a_mod_8) ^ constrained_bits ^ 2
    // Since constrained_bits are more significant we want to minimize constrained_bits.
    // Once we set constrained_bits, a_mod_8 has a fixed value
    // Search through all possible values of constrained_bits and find the min value possible for reg_a.

    let (_registers, program) = parse(input);
    let expected_output = program.clone();

    // Keep track of min output
    let mut min_reg_a = None;

    // BFS search
    let mut queue = VecDeque::new();
    let start_state = State {
        bits: Bits::new(),
        output_idx: 0,
    };
    queue.push_back(start_state);
    while let Some(State { bits, output_idx }) = queue.pop_front() {
        // dbg!(&bits.to_string());
        if output_idx == expected_output.len() {
            // We've reached the end of the program with no contradictions
            let reg_a = bits.calc();
            if min_reg_a.is_none() || reg_a < min_reg_a.unwrap() {
                min_reg_a = Some(reg_a);
            }
            continue;
        }

        let val = expected_output[output_idx];
        for constrained_bits in 0..8 {
            let curr_bits_idx = output_idx * 3;
            let a_mod_8 = constrained_bits ^ val ^ 2;
            let constrained_bits_idx = curr_bits_idx + (a_mod_8 ^ 6) as usize;

            // Try setting the current bits and the constrained bits
            // If there is a contradiction, this number is not possible and we can move on
            let Some(new_bits) = bits
                .set_3_bits(curr_bits_idx, a_mod_8)
                .and_then(|bits| bits.set_3_bits(constrained_bits_idx, constrained_bits))
            else {
                continue;
            };

            let new_state = State {
                bits: new_bits,
                output_idx: output_idx + 1,
            };

            queue.push_back(new_state);
        }
    }

    min_reg_a
}

#[derive(Clone, Debug)]
struct State {
    bits: Bits,
    output_idx: usize,
}

#[derive(Clone, Copy, Debug)]
enum Bit {
    Unset,     // bit can have any value
    Set(bool), // constrains the bit to this value
}

impl Bit {
    fn unwrap(&self) -> u64 {
        match &self {
            Bit::Unset => 0,
            Bit::Set(val) => *val as u64,
        }
    }

    // Check if there is a contradiction
    fn check(&self, other: Self) -> bool {
        match (&self, &other) {
            (Bit::Set(val1), Bit::Set(val2)) => val1 == val2,
            _ => true,
        }
    }
}

// max length is 3 * 16 (length of program) + 7
const MAX_LEN_BITS: usize = 3 * 16 + 7;

#[derive(Clone, Debug)]
struct Bits {
    // LSB at index 0
    bits: [Bit; MAX_LEN_BITS],
}

impl Bits {
    fn new() -> Self {
        Self {
            bits: [Bit::Unset; MAX_LEN_BITS],
        }
    }

    // If there is a contradiction with the existing bits, return None
    fn set_3_bits(&self, idx: usize, val: u8) -> Option<Self> {
        assert!(idx < MAX_LEN_BITS);
        assert!(val < 8);

        let mut new_bits = self.bits;
        new_bits[idx + 2] = Bit::Set(val & (1 << 2) != 0);
        new_bits[idx + 1] = Bit::Set(val & (1 << 1) != 0);
        new_bits[idx] = Bit::Set(val & 1 != 0);

        // check for contradiction
        for i in 0..3 {
            if !self.bits[idx + i].check(new_bits[idx + i]) {
                return None;
            }
        }

        let new_bits = Self { bits: new_bits };

        Some(new_bits)
    }

    fn calc(&self) -> u64 {
        let mut result = 0;
        for bit in self.bits.into_iter().rev() {
            result = (result << 1) + bit.unwrap();
        }

        result
    }
}

impl Display for Bits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for bit in self.bits.into_iter().rev() {
            let c = match bit {
                Bit::Unset => '.',
                Bit::Set(val) => {
                    if val {
                        '1'
                    } else {
                        '0'
                    }
                }
            };

            s.push(c);
        }

        f.write_str(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    // Check answer for part 2
    #[test]
    #[ignore = "depends on actual program input"]
    fn test_part_two() {
        let reg_a = 90938893795561;
        let (mut registers, program) = parse(&advent_of_code::template::read_file("inputs", DAY));
        registers[0] = reg_a;
        assert_eq!(run_program(&mut registers, program.clone()), program);
    }
}
