use std::{collections::HashMap, fs};

const MAX_STEP: isize = 100; // where 100 == 0 basically

struct Dial {
    number: usize,
    counts: HashMap<usize, usize>,
    counts_crossed_zero: usize,
}

impl Dial {
    fn new(number: usize) -> Dial {
        Dial {
            number,
            counts: HashMap::new(),
            counts_crossed_zero: 0,
        }
    }

    fn rotate(&mut self, steps: isize) {
        let prev_number = self.number as isize;
        let new_number = prev_number + steps;

        self.number = new_number.rem_euclid(MAX_STEP) as usize;

        // Count how many times we cross 0 during rotation.
        let wraps = if steps > 0 {
            new_number.div_euclid(MAX_STEP)
        } else {
            let ceil_div = |x: isize, m: isize| -> isize { -(-x).div_euclid(m) };
            ceil_div(prev_number, MAX_STEP) - ceil_div(new_number, MAX_STEP)
        };

        self.counts_crossed_zero += wraps.abs() as usize;
        *self.counts.entry(self.number).or_insert(0) += 1;
    }

    fn apply_command(&mut self, command: &str) {
        dbg!(command);
        let rotation = command
            .chars()
            .next()
            .expect("Command must have a character as first element");
        let number = command[1..]
            .parse::<isize>()
            .expect("Command must have a number as second element");
        match rotation {
            'L' => self.rotate(-1 * number),
            'R' => self.rotate(number),
            rot => panic!("Unknown command {rot}"),
        }
    }

    fn apply_commands(&mut self, commands: &str) {
        for command in commands.split_whitespace() {
            self.apply_command(command);
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input/day1/input.txt").expect("Could not read input");
    let mut dial = Dial::new(50);
    dial.apply_commands(&input);
    dbg!("Dial number is now {}", dial.number);
    dbg!("Counts of 0: {}", dial.counts.get(&0).unwrap());
    dbg!("Counts crossed zero: {}", dial.counts_crossed_zero);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_right() {
        let mut dial = Dial::new(0);
        dial.rotate(5);
        assert_eq!(dial.number, 5);
    }

    #[test]
    fn test_turn_overflow_right() {
        let mut dial = Dial::new(99);
        dial.rotate(1);
        assert_eq!(dial.number, 0);
    }

    #[test]
    fn test_turn_overflow_left() {
        let mut dial = Dial::new(0);
        dial.rotate(-1);
        assert_eq!(dial.number, 99);
    }

    #[test]
    fn test_apply_command_left_overflow() {
        let mut dial = Dial::new(50);
        dial.apply_command("L68");
        assert_eq!(dial.number, 82);
    }

    #[test]
    fn test_apply_command_count_zero() {
        let mut dial = Dial::new(50);
        dial.apply_command("R1000");
        assert_eq!(dial.number, 50);
        assert_eq!(dial.counts_crossed_zero, 10);
    }

    #[test]
    fn test_input() {
        let input = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let mut dial = Dial::new(50);
        dial.apply_commands(input);
        assert_eq!(dial.number, 32);
        assert_eq!(dial.counts.get(&0).unwrap(), &3);
        assert_eq!(dial.counts_crossed_zero, 6);
    }

    // === EDGE CASE TESTS ===

    #[test]
    fn test_start_at_zero_go_left_no_cross() {
        // Starting at 0 and going left should NOT cross 0 (going from 0 to 99)
        let mut dial = Dial::new(0);
        dial.rotate(-1);
        assert_eq!(dial.number, 99);
        assert_eq!(dial.counts_crossed_zero, 0);
    }

    #[test]
    fn test_start_at_zero_go_right_no_cross() {
        // Starting at 0 and going right should NOT cross 0
        let mut dial = Dial::new(0);
        dial.rotate(1);
        assert_eq!(dial.number, 1);
        assert_eq!(dial.counts_crossed_zero, 0);
    }

    #[test]
    fn test_land_exactly_on_zero_from_left() {
        // Going from 1 to 0 (rotating left by 1)
        let mut dial = Dial::new(1);
        dial.rotate(-1);
        assert_eq!(dial.number, 0);
        // landing on 0 "crosses" 0
        assert_eq!(dial.counts_crossed_zero, 1);
    }

    #[test]
    fn test_land_exactly_on_zero_from_right() {
        // Going from 99 to 0 (rotating right by 1) - crosses 0
        let mut dial = Dial::new(99);
        dial.rotate(1);
        assert_eq!(dial.number, 0);
        assert_eq!(dial.counts_crossed_zero, 1);
    }

    #[test]
    fn test_full_rotation_right() {
        // A full 100-step rotation right should cross 0 exactly once
        let mut dial = Dial::new(50);
        dial.rotate(100);
        assert_eq!(dial.number, 50);
        assert_eq!(dial.counts_crossed_zero, 1);
    }

    #[test]
    fn test_full_rotation_left() {
        // A full 100-step rotation left should cross 0 exactly once
        let mut dial = Dial::new(50);
        dial.rotate(-100);
        assert_eq!(dial.number, 50);
        assert_eq!(dial.counts_crossed_zero, 1);
    }

    #[test]
    fn test_multiple_rotations_right() {
        // 350 steps from 50 = 3 full rotations + 50 steps, crossing 0 four times
        // 50 -> 100(cross) -> 200(cross) -> 300(cross) -> 400(cross) -> ends at 0
        let mut dial = Dial::new(50);
        dial.rotate(350);
        assert_eq!(dial.number, 0);
        assert_eq!(dial.counts_crossed_zero, 4);
    }

    #[test]
    fn test_multiple_rotations_left() {
        // -350 steps from 50, crossing 0 multiple times going backwards
        let mut dial = Dial::new(50);
        dial.rotate(-350);
        assert_eq!(dial.number, 0);
        assert_eq!(dial.counts_crossed_zero, 4);
    }

    #[test]
    fn test_from_99_large_right_rotation() {
        // From 99, rotate right 2 - should cross 0 once, land on 1
        let mut dial = Dial::new(99);
        dial.rotate(2);
        assert_eq!(dial.number, 1);
        assert_eq!(dial.counts_crossed_zero, 1);
    }

    #[test]
    fn test_from_1_large_left_rotation() {
        // From 1, rotate left 2 - should cross 0 once, land on 99
        let mut dial = Dial::new(1);
        dial.rotate(-2);
        assert_eq!(dial.number, 99);
        assert_eq!(dial.counts_crossed_zero, 1);
    }

    #[test]
    fn test_zero_rotation() {
        // Rotating by 0 should not cross anything
        let mut dial = Dial::new(50);
        dial.rotate(0);
        assert_eq!(dial.number, 50);
        assert_eq!(dial.counts_crossed_zero, 0);
    }

    #[test]
    fn test_zero_rotation_starting_at_zero() {
        // Rotating by 0 starting at 0
        let mut dial = Dial::new(0);
        dial.rotate(0);
        assert_eq!(dial.number, 0);
        assert_eq!(dial.counts_crossed_zero, 0);
    }

    #[test]
    fn test_exactly_to_boundary_no_cross() {
        // From 50, rotate right 49 -> land on 99, no crossing
        let mut dial = Dial::new(50);
        dial.rotate(49);
        assert_eq!(dial.number, 99);
        assert_eq!(dial.counts_crossed_zero, 0);
    }

    #[test]
    fn test_exactly_past_boundary_one_cross() {
        // From 50, rotate right 50 -> land on 0, one crossing
        let mut dial = Dial::new(50);
        dial.rotate(50);
        assert_eq!(dial.number, 0);
        assert_eq!(dial.counts_crossed_zero, 1);
    }

    #[test]
    fn test_sequential_small_rotations_accumulate() {
        // Multiple small rotations that together cross 0
        let mut dial = Dial::new(95);
        dial.rotate(3); // 95 -> 98, no cross
        assert_eq!(dial.counts_crossed_zero, 0);
        dial.rotate(3); // 98 -> 1, cross!
        assert_eq!(dial.counts_crossed_zero, 1);
        dial.rotate(3); // 1 -> 4, no cross
        assert_eq!(dial.counts_crossed_zero, 1);
    }

    #[test]
    fn test_back_and_forth_across_zero() {
        // Cross 0 going right, then cross again going left
        let mut dial = Dial::new(99);
        dial.rotate(2); // 99 -> 1, cross
        assert_eq!(dial.number, 1);
        assert_eq!(dial.counts_crossed_zero, 1);
        dial.rotate(-2); // 1 -> 99, cross again
        assert_eq!(dial.number, 99);
        assert_eq!(dial.counts_crossed_zero, 2);
    }

    #[test]
    fn test_start_at_zero_full_rotation_back_to_zero() {
        // From 0, full rotation right ends at 0
        let mut dial = Dial::new(0);
        dial.rotate(100);
        assert_eq!(dial.number, 0);
        assert_eq!(dial.counts_crossed_zero, 1);
    }

    #[test]
    fn test_start_at_zero_full_rotation_left_back_to_zero() {
        // From 0, full rotation left ends at 0
        let mut dial = Dial::new(0);
        dial.rotate(-100);
        assert_eq!(dial.number, 0);
        assert_eq!(dial.counts_crossed_zero, 1);
    }
}
