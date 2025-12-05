use std::ops::RangeInclusive;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Default, PartialEq, Eq)]
struct ParsedInput {
    ranges: Vec<RangeInclusive<u64>>,
    numbers: Vec<u64>,
}

fn parse_input(input: impl AsRef<str>) -> ParsedInput {
    let input = input.as_ref();
    let (ranges_raw, numbers_raw) = input
        .split_once("\n\n")
        .expect("input should contain ranges and numbers delimited by an empty line");

    let mut parsed_input = ParsedInput::default();

    for range_raw in ranges_raw.lines() {
        let (start_raw, end_raw) = range_raw
            .split_once('-')
            .expect("range should have format: start-end");

        parsed_input.ranges.push(RangeInclusive::new(
            start_raw
                .parse()
                .expect("range start should be a valid number"),
            end_raw.parse().expect("range end should be a valid number"),
        ))
    }

    for number_raw in numbers_raw.lines() {
        parsed_input
            .numbers
            .push(number_raw.parse().expect("number should be parseable"))
    }

    parsed_input
}

#[cfg(test)]
mod tests {
    use crate::{ParsedInput, parse_input};

    #[test]
    fn test_parse_input() {
        let example_input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(
            parse_input(example_input),
            ParsedInput {
                ranges: vec![3..=5, 10..=14, 16..=20, 12..=18],
                numbers: vec![1, 5, 8, 11, 17, 32],
            }
        );
    }
}
