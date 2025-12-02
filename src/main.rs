use std::{collections::HashMap, fs};

const MAX_STEP: isize = 100; // where 100 == 0 basically

struct Dial {
    number: usize,
    counts: HashMap<usize, usize>,
}

impl Dial {
    fn new(number: usize) -> Dial {
        Dial {
            number,
            counts: HashMap::new(),
        }
    }

    fn rotate(&mut self, steps: isize) {
        self.number = (self.number as isize + steps).rem_euclid(MAX_STEP) as usize;

        *self.counts.entry(self.number).or_insert(0) += 1;
        dbg!("Dial number is now {}", self.number);
    }

    fn apply_command(&mut self, command: &str) {
        dbg!("Applying command {command}");
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
    }
}
