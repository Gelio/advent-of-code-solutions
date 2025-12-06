use std::io::{Read, stdin};

use aoc_2025_06::part1;

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("input should be valid");

    let parsed_input_part1 = part1::parse_input(input.lines()).expect("input should be valid");
    println!("Part 1: {}", part1::solve(&parsed_input_part1));
}
