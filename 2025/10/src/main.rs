use std::{io::stdin, str::FromStr};

fn main() {
    let problems: Vec<Problem> = stdin()
        .lines()
        .map(|line| {
            let line = line.expect("line should be valid");
            line.parse::<Problem>()
                .map_err(|err| format!("failed to parse line '{line}': {err}"))
                .expect("input should be valid")
        })
        .collect();

    println!("Part 1: {}", solve_part1(&problems));
}

fn solve_part1(problems: &Vec<Problem>) -> u32 {
    problems.iter().map(solve_problem_part1).sum()
}

fn solve_problem_part1(problem: &Problem) -> u32 {
    todo!()
}

// Problem represents a single line of the input
#[derive(Debug, Clone, PartialEq, Eq)]
struct Problem {
    initial_lights_state: LightsState,
    button_presses: Vec<ButtonPress>,
}

impl FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Example:
        // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

        let mut lights_state: Option<LightsState> = None;
        let mut button_presses: Vec<ButtonPress> = Vec::new();

        for part in s.split_whitespace() {
            match part.chars().nth(0) {
                Some('[') => {
                    if !part.ends_with(']') {
                        return Err("missing closing ']' in lights state".to_string());
                    }
                    let state_str = &part[1..part.len() - 1];
                    let state: LightsState = state_str.parse()?;
                    if lights_state.is_some() {
                        return Err("multiple lights states found".to_string());
                    }
                    lights_state = Some(state);
                }
                Some('(') => {
                    if !part.ends_with(')') {
                        return Err("missing closing ')' in button press".to_string());
                    }
                    let numbers_str = &part[1..part.len() - 1];
                    let numbers = parse_numbers(numbers_str)?;
                    let lights_len = lights_state
                        .as_ref()
                        .ok_or("button press found before lights state")?
                        .len;
                    let button_press = ButtonPress::new(&numbers, lights_len);
                    button_presses.push(button_press);
                }
                Some('{') => {
                    if !part.ends_with('}') {
                        return Err("missing closing '}' in ignored part".to_string());
                    }
                    // Ignore this part (the joltage) for now.
                }
                Some(_) => {
                    return Err(format!("unexpected part in input: '{part}'"));
                }
                None => {
                    return Err("empty part in input".to_string());
                }
            }
        }

        let lights_state = lights_state.ok_or("missing lights state")?;
        if button_presses.is_empty() {
            return Err("no button presses found".to_string());
        }

        Ok(Problem {
            initial_lights_state: lights_state,
            button_presses,
        })
    }
}

type LightsStateBits = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LightsState {
    // Example: .##. means: off, on, on, off.
    // Represented as bits in a u16: 0b0110 = 6 (since the light position matters)
    bits: LightsStateBits,
    // Size of the input (how many lights there are).
    // Example: for .##. this would be 4.
    len: usize,
}

impl FromStr for LightsState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut state = 0u16;
        let len = s.len();
        for (i, c) in s.chars().enumerate() {
            match c {
                '.' => (),
                '#' => state |= 1 << (len - 1 - i),
                _ => return Err(format!("Invalid character '{c}' in input")),
            }
        }
        Ok(LightsState { bits: state, len })
    }
}

fn parse_numbers(input: &str) -> Result<Vec<u32>, String> {
    input
        .split(',')
        .map(|num_str| {
            num_str
                .trim()
                .parse::<u32>()
                .map_err(|e| format!("Failed to parse number '{num_str}': {e:?}"))
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ButtonPress {
    lights_switched_bits: LightsStateBits,
}

impl ButtonPress {
    fn new(lights_switched: &Vec<u32>, lights_len: usize) -> Self {
        let mut bits = 0u16;
        for &light in lights_switched {
            bits |= 1 << (lights_len - 1 - light as usize);
        }
        ButtonPress {
            lights_switched_bits: bits,
        }
    }
}

fn toggle_lights(state: &LightsState, button: &ButtonPress) -> LightsState {
    LightsState {
        bits: state.bits ^ button.lights_switched_bits,
        len: state.len,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lights_state_from_str() {
        let state: LightsState = ".##.".parse().unwrap();
        assert_eq!(state.bits, 0b0110);
        assert_eq!(state.len, 4);

        let state: LightsState = "...#.".parse().unwrap();
        assert_eq!(state.bits, 0b00010);
        assert_eq!(state.len, 5);
    }

    #[test]
    fn test_parse_numbers() {
        assert_eq!(parse_numbers("1,2,3,4"), Ok(vec![1, 2, 3, 4,]));
    }

    #[test]
    fn test_button_press_new() {
        let button = ButtonPress::new(&vec![1, 3], 5);
        assert_eq!(button.lights_switched_bits, 0b01010);
    }

    #[test]
    fn test_toggle_lights() {
        let state: LightsState = ".##.".parse().unwrap();
        let button = ButtonPress::new(&vec![0, 2], state.len);
        let new_state = toggle_lights(&state, &button);
        assert_eq!(new_state.bits, 0b1100);
    }

    #[test]
    fn test_problem_from_str() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let problem: Problem = input.parse().unwrap();
        assert_eq!(problem.initial_lights_state.bits, 0b0110);
        assert_eq!(
            problem.button_presses,
            vec![
                ButtonPress::new(&vec![3], 4),
                ButtonPress::new(&vec![1, 3], 4),
                ButtonPress::new(&vec![2], 4),
                ButtonPress::new(&vec![2, 3], 4),
                ButtonPress::new(&vec![0, 2], 4),
                ButtonPress::new(&vec![0, 1], 4),
            ]
        );
    }

    #[test]
    fn test_solve_problem_part1() {
        let problem: Problem = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
            .parse()
            .unwrap();
        assert_eq!(solve_problem_part1(&problem), 2);

        let problem: Problem = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"
            .parse()
            .unwrap();
        assert_eq!(solve_problem_part1(&problem), 3);

        let problem: Problem = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
            .parse()
            .unwrap();
        assert_eq!(solve_problem_part1(&problem), 2);
    }
}
