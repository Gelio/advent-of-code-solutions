use std::{io::stdin, str::FromStr};

fn main() {
    let (part1_result, part2_result) =
        solve(stdin().lines().map(|l| l.expect("Could not read line")));
    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
}

fn solve(input: impl IntoIterator<Item = String>) -> (u32, u32) {
    let mut position: i32 = 50;
    let mut at_zero = 0;
    let mut crossed_zero_times = 0;

    for line in input {
        let dial_move: DialMove = line.parse().expect("Cannot parse dial move");
        let crossed_zero_times_during_move = move_dial(&mut position, &dial_move);
        crossed_zero_times += crossed_zero_times_during_move;

        if position == 0 {
            at_zero += 1;
        }
    }

    (at_zero, crossed_zero_times)
}

// Returns the number of times the dial crossed the 0 position as part of this move.
fn move_dial(position: &mut i32, dial_move: &DialMove) -> u32 {
    match dial_move.direction {
        TurnDirection::Left => {
            let initially_at_zero = *position == 0;
            *position -= dial_move.distance as i32;
            if *position < 0 {
                let mut crosses_zero_times = u32::div_ceil(-*position as u32, DIAL_SIZE);
                *position += (crosses_zero_times * DIAL_SIZE) as i32;
                if initially_at_zero {
                    crosses_zero_times -= 1
                }
                if *position == 0 {
                    crosses_zero_times += 1
                }
                crosses_zero_times
            } else if *position == 0 {
                1
            } else {
                0
            }
        }
        TurnDirection::Right => {
            *position += dial_move.distance as i32;
            let crosses_zero_times = (*position as u32) / DIAL_SIZE;
            *position -= (crosses_zero_times * DIAL_SIZE) as i32;
            crosses_zero_times
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
    use crate::{move_dial, solve};

    const SAMPLE_INPUT: &str = "L68
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

    #[test]
    fn test_part1() {
        let (result, _) = solve(SAMPLE_INPUT.lines().map(str::to_string));

        assert_eq!(result, 3);
    }

    #[test]
    fn test_part2() {
        let (_, result) = solve(SAMPLE_INPUT.lines().map(str::to_string));

        assert_eq!(result, 6);
    }

    #[test]
    fn move_dial_left_underflow() {
        let mut position = 0;
        let crosses_zero_times =
            move_dial(&mut position, &"L1".parse().expect("Cannot parse move"));
        assert_eq!(position, 99);
        assert_eq!(crosses_zero_times, 0);
    }

    #[test]
    fn move_dial_left_underflow_multiple_start_from_zero() {
        let mut position = 0;
        let crosses_zero_times =
            move_dial(&mut position, &"L101".parse().expect("Cannot parse move"));
        assert_eq!(position, 99);
        assert_eq!(crosses_zero_times, 1);
    }

    #[test]
    fn move_dial_left_underflow_multiple() {
        let mut position = 1;
        let crosses_zero_times =
            move_dial(&mut position, &"L102".parse().expect("Cannot parse move"));
        assert_eq!(position, 99);
        assert_eq!(crosses_zero_times, 2);
    }

    #[test]
    fn move_dial_left_underflow_multiple_to_zero() {
        let mut position = 1;
        let crosses_zero_times =
            move_dial(&mut position, &"L101".parse().expect("Cannot parse move"));
        assert_eq!(position, 0);
        assert_eq!(crosses_zero_times, 2);
    }

    #[test]
    fn move_dial_right_underflow() {
        let mut position = 99;
        let crosses_zero_times =
            move_dial(&mut position, &"R1".parse().expect("Cannot parse move"));
        assert_eq!(position, 0);
        assert_eq!(crosses_zero_times, 1);
    }

    #[test]
    fn move_dial_right_underflow_multiple() {
        let mut position = 99;
        let crosses_zero_times =
            move_dial(&mut position, &"R101".parse().expect("Cannot parse move"));
        assert_eq!(position, 0);
        assert_eq!(crosses_zero_times, 2);
    }
}
