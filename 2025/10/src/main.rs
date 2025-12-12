use std::{
    collections::{HashMap, VecDeque},
    io::stdin,
    str::FromStr,
};

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
    println!("Part 2: {}", solve_part2(&problems));
}

fn solve_part1(problems: &Vec<Problem>) -> u32 {
    problems.iter().map(solve_problem_part1).sum()
}

fn solve_part2(problems: &Vec<Problem>) -> u32 {
    problems.iter().map(solve_problem_part2).sum()
}

fn solve_problem_part1(problem: &Problem) -> u32 {
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct GraphNode {
        state: LightsStateBits,
        distance_from_start: u32,
    }

    let mut nodes_to_visit: VecDeque<GraphNode> = VecDeque::new();
    nodes_to_visit.push_back(GraphNode {
        state: 0,
        distance_from_start: 0,
    });
    let mut visited_nodes: HashMap<LightsStateBits, GraphNode> = HashMap::new();

    while !visited_nodes.contains_key(&problem.final_lights_state.bits) {
        let node = nodes_to_visit.pop_front().expect(
            "nodes to visit queue should not be exhausted before reaching the goal lights state",
        );

        for button_press in problem.button_presses.iter() {
            let next_node_state = toggle_lights(node.state, button_press);

            if visited_nodes.contains_key(&next_node_state) {
                continue;
            }

            let next_node = GraphNode {
                state: next_node_state,
                distance_from_start: node.distance_from_start + 1,
            };
            visited_nodes.insert(next_node_state, next_node.clone());

            nodes_to_visit.push_back(next_node);
        }
    }

    visited_nodes
        .get(&problem.final_lights_state.bits)
        .expect("final node was found")
        .distance_from_start
}

fn solve_problem_part2(problem: &Problem) -> u32 {
    let lights_count = problem.final_lights_state.len;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct NodeToVisit {
        joltage_ratings: Vec<u32>,
        distance_from_start: u32,
    }

    // TODO: use a BinaryHeap for a priority queue.
    // Estimate metric to finish: minimum_distance_to_finish = (expected_joltage_ratings - joltage_ratings).sum()
    // After finding *some* solution, keep trying. Maybe there is a shorter path.
    // Keep trying until the best solution found so far is shorter than
    // distance_from_start + minimum_distance_to_finish
    let mut nodes_to_visit: VecDeque<NodeToVisit> = VecDeque::new();
    nodes_to_visit.push_back(NodeToVisit {
        joltage_ratings: vec![0; lights_count],
        distance_from_start: 0,
    });
    let mut visited_nodes_distance_from_start: HashMap<Vec<u32>, u32> = HashMap::new();

    while !visited_nodes_distance_from_start.contains_key(&problem.expected_joltage_ratings) {
        let node = nodes_to_visit.pop_front().expect(
            "nodes to visit queue should not be exhausted before reaching the goal joltage ratings",
        );
        // println!(
        //     "Visiting {:?} (dist = {})",
        //     node.joltage_ratings, node.distance_from_start
        // );

        for button_press in problem.button_presses.iter() {
            let mut next_node_joltage = node.joltage_ratings.clone();
            increase_joltage(&mut next_node_joltage, button_press);

            if is_joltage_too_high(&next_node_joltage, &problem.expected_joltage_ratings) {
                continue;
            }

            if visited_nodes_distance_from_start.contains_key(&next_node_joltage) {
                continue;
            }

            let next_node = NodeToVisit {
                joltage_ratings: next_node_joltage.clone(),
                distance_from_start: node.distance_from_start + 1,
            };
            visited_nodes_distance_from_start
                .insert(next_node_joltage, node.distance_from_start + 1);

            nodes_to_visit.push_back(next_node);
        }
    }

    // println!(
    //     "Solved problem, visited {} nodes, wanted to visit {} more nodes",
    //     visited_nodes_distance_from_start.len(),
    //     nodes_to_visit.len()
    // );

    *visited_nodes_distance_from_start
        .get(&problem.expected_joltage_ratings)
        .expect("final node was found")
}

fn increase_joltage(joltage: &mut Vec<u32>, button_press: &ButtonPress) {
    for light_index in button_press.lights_switched.iter() {
        joltage[*light_index as usize] += 1;
    }
}

fn is_joltage_too_high(joltage: &Vec<u32>, expected_joltage: &Vec<u32>) -> bool {
    for (current, expected) in joltage.iter().zip(expected_joltage.iter()) {
        if current > expected {
            return true;
        }
    }

    false
}

// Problem represents a single line of the input
#[derive(Debug, Clone, PartialEq, Eq)]
struct Problem {
    final_lights_state: LightsState,
    button_presses: Vec<ButtonPress>,
    expected_joltage_ratings: Vec<u32>,
}

impl FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Example:
        // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

        let mut lights_state: Option<LightsState> = None;
        let mut button_presses: Vec<ButtonPress> = Vec::new();
        let mut expected_joltage_ratings: Option<Vec<u32>> = None;

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
                    let button_press = ButtonPress::new(numbers, lights_len);
                    button_presses.push(button_press);
                }
                Some('{') => {
                    if !part.ends_with('}') {
                        return Err("missing closing '}' in ignored part".to_string());
                    }

                    if expected_joltage_ratings.is_some() {
                        return Err("multiple expected joltage ratings found".to_string());
                    }

                    let numbers = parse_numbers(&part[1..part.len() - 1])
                        .map_err(|err| format!("cannot parse joltage levels in {part}: {err:?}"))?;
                    expected_joltage_ratings = Some(numbers);
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
        let expected_joltage_ratings =
            expected_joltage_ratings.ok_or("missing expected joltage ratings")?;
        if expected_joltage_ratings.len() != lights_state.len {
            return Err(format!(
                "expected joltage ratings length {} does not match lights count {}",
                expected_joltage_ratings.len(),
                lights_state.len
            ));
        }

        Ok(Problem {
            final_lights_state: lights_state,
            button_presses,
            expected_joltage_ratings,
        })
    }
}

type LightsStateBits = u16;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct LightsState {
    // Example: .##. means: off, on, on, off.
    // Represented as bits in a u16: 0b0110 = 6 (since the light position matters)
    bits: LightsStateBits,
    // Size of the input (how many lights there are).
    // Example: for .##. this would be 4.
    // TODO: remove this field from the struct since it never changes for the entire problem.
    // It is unnecessarily cloned and copied.
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
    lights_switched: Vec<u32>,
}

impl ButtonPress {
    fn new(lights_switched: Vec<u32>, lights_len: usize) -> Self {
        let mut bits = 0u16;
        for &light in lights_switched.iter() {
            bits |= 1 << (lights_len - 1 - light as usize);
        }
        ButtonPress {
            lights_switched_bits: bits,
            lights_switched,
        }
    }
}

fn toggle_lights(state: LightsStateBits, button: &ButtonPress) -> LightsStateBits {
    state ^ button.lights_switched_bits
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
        let button = ButtonPress::new(vec![1, 3], 5);
        assert_eq!(button.lights_switched_bits, 0b01010);
    }

    #[test]
    fn test_toggle_lights() {
        let state: LightsState = ".##.".parse().unwrap();
        let button = ButtonPress::new(vec![0, 2], state.len);
        let new_state = toggle_lights(state.bits, &button);
        assert_eq!(new_state, 0b1100);
    }

    #[test]
    fn test_problem_from_str() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let problem: Problem = input.parse().unwrap();
        assert_eq!(problem.final_lights_state.bits, 0b0110);
        assert_eq!(
            problem.button_presses,
            vec![
                ButtonPress::new(vec![3], 4),
                ButtonPress::new(vec![1, 3], 4),
                ButtonPress::new(vec![2], 4),
                ButtonPress::new(vec![2, 3], 4),
                ButtonPress::new(vec![0, 2], 4),
                ButtonPress::new(vec![0, 1], 4),
            ]
        );
        assert_eq!(problem.expected_joltage_ratings, vec![3, 5, 4, 7]);
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

    #[test]
    fn test_solve_problem_part2() {
        let problem: Problem = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
            .parse()
            .unwrap();
        assert_eq!(solve_problem_part2(&problem), 10);

        let problem: Problem = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"
            .parse()
            .unwrap();
        assert_eq!(solve_problem_part2(&problem), 12);

        let problem: Problem = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
            .parse()
            .unwrap();
        assert_eq!(solve_problem_part2(&problem), 11);
    }
}
