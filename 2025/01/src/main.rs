use std::{io::stdin, str::FromStr};

fn main() {
    let part1_result = part1(stdin().lines().map(|l| l.expect("Could not read line")));
    println!("Part 1: {part1_result}");
}

fn part1(input: impl IntoIterator<Item = String>) -> u32 {
    let mut position: i32 = 50;
    let mut at_zero = 0;

    for line in input {
        let dial_move: DialMove = line.parse().expect("Cannot parse dial move");
        move_dial(&mut position, &dial_move);

        if position == 0 {
            at_zero += 1;
        }
    }

    at_zero
}

fn move_dial(position: &mut i32, dial_move: &DialMove) {
    match dial_move.direction {
        TurnDirection::Left => {
            *position -= dial_move.distance as i32;
            if *position < 0 {
                *position += (u32::div_ceil(-*position as u32, DIAL_SIZE) * DIAL_SIZE) as i32;
            }
        }
        TurnDirection::Right => {
            *position += dial_move.distance as i32;
            *position %= DIAL_SIZE as i32;
        }
    }
}

#[derive(Debug)]
enum TurnDirection {
    Left,
    Right,
}

#[derive(Debug)]
struct DialMove {
    direction: TurnDirection,
    distance: u32,
}

const DIAL_SIZE: u32 = 100;

impl FromStr for DialMove {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(
            s.len() >= 2,
            "Line must have at least a direction and a one-digit distance"
        );
        let direction = match s.chars().nth(0) {
            Some('L') => TurnDirection::Left,
            Some('R') => TurnDirection::Right,
            _ => {
                return Err("Unsupported direction".to_string());
            }
        };

        let distance: u32 = s[1..]
            .parse()
            .map_err(|err| format!("Cannot parse distance {err:#?}"))?;

        Ok(DialMove {
            direction,
            distance,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{DialMove, move_dial, part1};

    #[test]
    fn test_part1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

        let result = part1(input.lines().map(str::to_string));

        assert_eq!(result, 3);
    }

    #[test]
    fn move_dial_left_underflow() {
        let mut position = 0;
        move_dial(&mut position, &"L1".parse().expect("Cannot parse move"));
        assert_eq!(position, 99);
    }

    #[test]
    fn move_dial_left_underflow_multiple() {
        let mut position = 0;
        move_dial(&mut position, &"L101".parse().expect("Cannot parse move"));
        assert_eq!(position, 99);
    }

    #[test]
    fn move_dial_left_underflow_multiple_to_zero() {
        let mut position = 1;
        move_dial(&mut position, &"L101".parse().expect("Cannot parse move"));
        assert_eq!(position, 0);
    }

    #[test]
    fn move_dial_right_underflow() {
        let mut position = 99;
        move_dial(&mut position, &"R1".parse().expect("Cannot parse move"));
        assert_eq!(position, 0);
    }

    #[test]
    fn move_dial_right_underflow_multiple() {
        let mut position = 99;
        move_dial(&mut position, &"R101".parse().expect("Cannot parse move"));
        assert_eq!(position, 0);
    }
}
