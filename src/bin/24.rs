use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    str::FromStr,
};

use itertools::Itertools;

advent_of_code::solution!(24);

struct Circuit {
    values: HashMap<String, bool>,
    // { output name: gate }
    gates: HashMap<String, Gate>,
}

impl Circuit {
    fn new(initial_values: HashMap<String, bool>, gates: Vec<Gate>) -> Self {
        Self {
            values: initial_values,
            gates: gates
                .into_iter()
                .map(|gate| (gate.out.clone(), gate))
                .collect(),
        }
    }

    fn solve_for(&self, name: &str) -> bool {
        if let Some(out) = self.values.get(name) {
            return *out;
        }

        let gate = self.gates.get(name).unwrap();

        let in1 = self.solve_for(&gate.in1);
        let in2 = self.solve_for(&gate.in2);

        gate.get_output(in1, in2)
    }

    // This does some redundant computations but is fast enough for part 1
    fn get_z_value(&self) -> u64 {
        let mut i = 0;
        let mut value = 0;
        loop {
            let name = format!("z{i:0>2}");
            if !self.gates.contains_key(&name) {
                break;
            }

            let bit = self.solve_for(&name);
            if bit {
                value += 1 << i;
            }

            i += 1;
        }

        value
    }
}

#[derive(Clone, Debug)]
struct Gate {
    in1: String,
    in2: String,
    op: Op,
    out: String,
}

impl Gate {
    fn get_output(&self, in1: bool, in2: bool) -> bool {
        self.op.apply(in1, in2)
    }

    fn set_out(&mut self, new_out: String) {
        self.out = new_out;
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Op {
    AND,
    OR,
    XOR,
}

impl Op {
    fn apply(&self, in1: bool, in2: bool) -> bool {
        match self {
            Self::AND => in1 & in2,
            Self::OR => in1 | in2,
            Self::XOR => in1 ^ in2,
        }
    }
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::AND),
            "OR" => Ok(Self::OR),
            "XOR" => Ok(Self::XOR),
            s => Err(format!("Unrecognized operation {s}")),
        }
    }
}

fn parse(input: &str) -> Circuit {
    let (initial_values, gates) = input
        .split("\n\n")
        .collect_tuple()
        .unwrap_or_else(|| panic!("Expected two blocks in input"));

    let initial_values = initial_values
        .trim()
        .lines()
        .map(parse_initial_value)
        .collect();
    let gates = gates.trim().lines().map(parse_gate).collect();

    Circuit::new(initial_values, gates)
}

fn parse_initial_value(line: &str) -> (String, bool) {
    let (name, value) = line
        .split(": ")
        .collect_tuple()
        .unwrap_or_else(|| panic!("Failed to split {line} as initial value"));

    let value = if value == "1" {
        true
    } else if value == "0" {
        false
    } else {
        panic!("Failed to parse value {value}");
    };

    (name.to_string(), value)
}

fn parse_gate(line: &str) -> Gate {
    let (in1, op, in2, _, out) = line
        .split_whitespace()
        .collect_tuple()
        .unwrap_or_else(|| panic!("Failed to split {line} as Gate"));

    Gate {
        in1: in1.to_string(),
        in2: in2.to_string(),
        op: Op::from_str(op).unwrap(),
        out: out.to_string(),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let circuit = parse(input);

    Some(circuit.get_z_value())
}

// Build a bunch of indexes on the gates so we can search in different ways
struct Gates {
    gates: Vec<Gate>,
    // All indexes refer to the gate's index in self.gates
    by_output: HashMap<String, usize>,
    by_op_and_input: HashMap<(Op, String), usize>,
    by_op_and_inputs: HashMap<(Op, String, String), usize>,
}

impl Gates {
    fn new(gates: Vec<Gate>) -> Self {
        let mut by_output = HashMap::new();
        let mut by_op_and_input = HashMap::new();
        let mut by_op_and_inputs = HashMap::new();

        for (i, gate) in gates.iter().enumerate() {
            by_output.insert(gate.out.clone(), i);

            by_op_and_input.insert((gate.op.clone(), gate.in1.clone()), i);
            by_op_and_input.insert((gate.op.clone(), gate.in2.clone()), i);

            by_op_and_inputs.insert((gate.op.clone(), gate.in1.clone(), gate.in2.clone()), i);
        }

        Self {
            gates,
            by_output,
            by_op_and_input,
            by_op_and_inputs,
        }
    }

    // swap out1 and out2
    fn swap(&mut self, out1: &str, out2: &str) {
        let i1 = *self.by_output.get(out1).unwrap();
        let i2 = *self.by_output.get(out2).unwrap();

        // update gates
        self.gates.get_mut(i1).unwrap().set_out(out2.to_string());
        self.gates.get_mut(i2).unwrap().set_out(out1.to_string());

        // update indexes
        self.by_output.insert(out1.to_string(), i2);
        self.by_output.insert(out2.to_string(), i1);
    }

    fn get_by_output(&self, output: &str) -> Option<&Gate> {
        self.by_output.get(output).and_then(|i| self.gates.get(*i))
    }

    fn get_by_op_and_input(&self, op: Op, in_: &str) -> Option<&Gate> {
        self.by_op_and_input
            .get(&(op, in_.to_string()))
            .and_then(|i| self.gates.get(*i))
    }

    fn get_by_op_and_inputs(&self, op: Op, in1: &str, in2: &str) -> Option<&Gate> {
        // inputs could be in any order so search for both
        let k1 = (op.clone(), in1.to_string(), in2.to_string());
        let k2 = (op, in2.to_string(), in1.to_string());
        self.by_op_and_inputs
            .get(&k1)
            .or_else(|| self.by_op_and_inputs.get(&k2))
            .and_then(|i| self.gates.get(*i))
    }
}

impl Debug for Gates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Gates").field("gates", &self.gates).finish()
    }
}

fn parse_gates(input: &str) -> Gates {
    // we only care about the gates
    let (_, gates) = input
        .split("\n\n")
        .collect_tuple()
        .unwrap_or_else(|| panic!("Expected two blocks in input"));

    let gates = gates.trim().lines().map(parse_gate).collect();
    Gates::new(gates)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut gates = parse_gates(input);

    // Ripple-carry adder (https://en.wikipedia.org/wiki/Adder_(electronics)#Full_adder)
    // x_i is bit i of x, same for y, z
    // C_i is the carry bit applied to bit i (generated at bit i-1)
    // bit z_i = x_i ^ y_i ^ C_i
    // C_(i+1) = (x_i & y_i) | (C_i & (x_i ^ y_i))

    let mut i = 0;
    let mut swaps = HashSet::new();

    // Carry bit generated from the previous loop iteration
    let mut c_i = String::new();

    loop {
        if swaps.len() == 8 {
            // Problem stated there are only 4 swaps total, so we can stop here
            break;
        }

        let z_i = format!("z{i:0>2}");
        let x_i = format!("x{i:0>2}");
        let y_i = format!("y{i:0>2}");

        if i == 0 {
            // z_1 = x_1 ^ y_1
            // Skip the check here (checked by hand)

            // figure out which register holds the next carry bit
            // special case since there is no C_0
            // C_1 = (x_i & y_i)
            c_i = gates
                .get_by_op_and_inputs(Op::AND, &x_i, &y_i)
                .unwrap()
                .out
                .clone();
        } else {
            // Standard case
            // First check: bit z_i = x_i ^ y_i ^ C_i
            let mut gate_z_i = gates.get_by_output(&z_i).unwrap().clone();
            // We always do (x_i ^ y_i) since we also use it later,
            // so one input should be C_i.

            // If none of the inputs are C_i then z_i must be wrong
            if gate_z_i.in1 != c_i && gate_z_i.in2 != c_i {
                // there should be another gate that is doing an XOR with C_i, which is the correct z_i
                gate_z_i = gates.get_by_op_and_input(Op::XOR, &c_i).unwrap().clone();
                swap(&mut gates, &mut swaps, &z_i, &gate_z_i.out);
            }

            // Find which gate the other input should be = (x_i ^ y_i)
            let mut x_i_xor_y_i = gates
                .get_by_op_and_inputs(Op::XOR, &x_i, &y_i)
                .unwrap()
                .out
                .clone();

            // Swap if needed
            if let Some(swapped) =
                swap_if_other_in_not_equal(gate_z_i, &c_i, &x_i_xor_y_i, &mut gates, &mut swaps)
            {
                // If we swap, then the variable actually holding x_i ^ y_i has changed, so update it
                x_i_xor_y_i = swapped;
            }

            // Next check: C_(i+1) = (x_i & y_i) | (C_i & (x_i ^ y_i))
            // Call a_i = (C_i & (x_i ^ y_i))
            let a_i = gates
                .get_by_op_and_inputs(Op::AND, &c_i, &x_i_xor_y_i)
                .unwrap()
                .out
                .clone();
            let x_i_and_y_i = gates
                .get_by_op_and_inputs(Op::AND, &x_i, &y_i)
                .unwrap()
                .out
                .clone();

            // Another gate c_i should be equal to a_i | x_i_and_y_i
            c_i = if let Some(gate) = gates.get_by_op_and_input(Op::OR, &a_i).cloned() {
                let c_i = gate.out.clone();
                swap_if_other_in_not_equal(gate, &a_i, &x_i_and_y_i, &mut gates, &mut swaps);

                c_i
            } else if let Some(gate) = gates.get_by_op_and_input(Op::OR, &x_i_and_y_i).cloned() {
                let c_i = gate.out.clone();
                swap_if_other_in_not_equal(gate, &x_i_and_y_i, &a_i, &mut gates, &mut swaps);

                c_i
            } else {
                panic!("Did not find a_i {a_i} or x_i_and_y_i {x_i_and_y_i} in gates");
            };
        }

        i += 1;
    }

    Some(sort_and_join_swaps(swaps))
}

// Return the other input that was swapped, if swapped
fn swap_if_other_in_not_equal(
    gate: Gate,
    known_input: &str,           // known input
    other_in_should_equal: &str, // other input should equal this
    gates: &mut Gates,
    swaps: &mut HashSet<String>,
) -> Option<String> {
    let other_in = if gate.in1 == known_input {
        gate.in2
    } else if gate.in2 == known_input {
        gate.in1
    } else {
        panic!("None of the inputs in {gate:?} are equal to known input {known_input}");
    };

    if other_in != other_in_should_equal {
        swap(gates, swaps, &other_in, other_in_should_equal);
        return Some(other_in);
    }

    None
}

fn swap(gates: &mut Gates, swaps: &mut HashSet<String>, out1: &str, out2: &str) {
    gates.swap(out1, out2);
    swaps.insert(out1.to_string());
    swaps.insert(out2.to_string());
}

fn sort_and_join_swaps(swaps: HashSet<String>) -> String {
    let mut v = Vec::from_iter(swaps);
    v.sort();
    v.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }
}
