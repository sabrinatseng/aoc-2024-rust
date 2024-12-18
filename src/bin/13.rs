use itertools::Itertools;

advent_of_code::solution!(13);

#[derive(Clone, Copy)]
struct Button {
    dx: usize,
    dy: usize,
}

impl Button {
    fn press_n_times(&self, n: usize) -> (usize, usize) {
        (self.dx * n, self.dy * n)
    }
}

struct ClawMachine {
    button_a: Button,
    button_b: Button,
    prize: (usize, usize),
}

impl ClawMachine {
    fn wins_prize(&self, a_presses: usize, b_presses: usize) -> bool {
        let (ax, ay) = self.button_a.press_n_times(a_presses);
        let (bx, by) = self.button_b.press_n_times(b_presses);

        (ax + bx, ay + by) == self.prize
    }

    fn update_prize_location_for_part_two(&mut self) {
        // the position of every prize is actually 10000000000000 higher on both the X and Y axis
        self.prize.0 += 10000000000000;
        self.prize.1 += 10000000000000;
    }

    fn solve(&self) -> Option<(usize, usize)> {
        let (prize_x, prize_y) = self.prize;
        let Button { dx: a_dx, dy: a_dy } = self.button_a;
        let Button { dx: b_dx, dy: b_dy } = self.button_b;

        // We can solve a linear system of equations:
        // a * a.dx + b * b.dx = prize_x (1)
        // a * a.dy + b * b.dy = prize_y (2)

        // Solve for a by multiply (1) by b.dy and (2) by b.dx to subtract out b:
        // a * a.dx * b.dy + b * b.dx * b.dy = prize_x * b.dy (3)
        // a * a.dy * b.dx + b * b.dx * b.dy = prize_y * b.dx (4)
        // subtract:
        // a * (a.dx * b.dy - a.dy * b.dx) = (prize_x * b.dy - prize_y * b.dx)
        // divide:
        // a = (prize_x * b.dy - prize_y * b.dx) / (a.dx * b.dy - a.dy * b.dx)
        // b = (prize_x - a * a.dx) / b.dx

        // We will just do integer division and abs_diff for simplicity (since the solution
        // is only valid if a and b are both integers), then double check the
        // solution at the end
        let a = (((prize_x * b_dy) as u128).abs_diff((prize_y * b_dx) as u128))
            .checked_div(((a_dx * b_dy) as u128).abs_diff((a_dy * b_dx) as u128))?;
        let b = ((prize_x as u128).abs_diff(a * a_dx as u128)).checked_div(b_dx as u128)?;

        if self.wins_prize(a as usize, b as usize) {
            return Some((a as usize, b as usize));
        }

        None
    }
}

fn parse(input: &str) -> Vec<ClawMachine> {
    input.split("\n\n").map(parse_claw_machine).collect()
}

fn parse_claw_machine(input: &str) -> ClawMachine {
    let (a, b, prize) = input
        .lines()
        .collect_tuple()
        .expect("Expected 3 input lines per claw machine");

    ClawMachine {
        button_a: parse_button(a),
        button_b: parse_button(b),
        prize: parse_prize(prize),
    }
}

fn parse_button(input: &str) -> Button {
    // Remove everything before the first number
    let start = "Button A: X+".len();
    let input = &input[start..];

    let dx_end = input
        .find(',')
        .unwrap_or_else(|| panic!("Did not find , in {input}"));
    let dy_start = input
        .find('+')
        .unwrap_or_else(|| panic!("Did not find second + in {input}"))
        + 1;

    let dx = input[..dx_end]
        .parse()
        .unwrap_or_else(|e| panic!("Failed to parse dx for {input}: {e}"));
    let dy = input[dy_start..]
        .parse()
        .unwrap_or_else(|e| panic!("Failed to parse dy for {input}: {e}"));

    Button { dx, dy }
}

fn parse_prize(input: &str) -> (usize, usize) {
    // Remove everything before the first number
    let start = "Prize: X=".len();
    let input = &input[start..];

    let x_end = input
        .find(',')
        .unwrap_or_else(|| panic!("Did not find , in {input}"));
    let y_start = input
        .find('=')
        .unwrap_or_else(|| panic!("Did not find second = in {input}"))
        + 1;

    let x = input[..x_end]
        .parse()
        .unwrap_or_else(|e| panic!("Failed to parse x for {input}: {e}"));
    let y = input[y_start..]
        .parse()
        .unwrap_or_else(|e| panic!("Failed to parse y for {input}: {e}"));

    (x, y)
}

pub fn part_one(input: &str) -> Option<u32> {
    let claw_machines = parse(input);

    let mut tokens = 0;
    for claw_machine in claw_machines {
        // Brute force - try all combinations of 100 button presses
        // Since a presses are more expensive, we will hit the cheapest solution first
        'outer: for a_presses in 0..100 {
            for b_presses in 0..100 {
                if claw_machine.wins_prize(a_presses, b_presses) {
                    tokens += 3 * a_presses + b_presses;
                    break 'outer;
                }
            }
        }
    }

    Some(tokens as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut claw_machines = parse(input);

    let mut tokens = 0;

    for claw_machine in claw_machines.iter_mut() {
        // Update prize location
        claw_machine.update_prize_location_for_part_two();

        if let Some((a, b)) = claw_machine.solve() {
            tokens += a * 3 + b;
        }
    }

    Some(tokens as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert!(result.is_some());
    }
}
