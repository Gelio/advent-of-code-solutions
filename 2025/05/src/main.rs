use std::{
    io::{Read, stdin},
    ops::RangeInclusive,
};

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("stdin should be valid input");
    let parsed_input = parse_input(&input);

    dbg!(parsed_input);
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

#[derive(Debug, Default)]
struct DisjointRanges {
    // Disjoint ranges in ascending order
    ranges: Vec<RangeInclusive<u64>>,
}

fn maybe_join_ranges(
    r1: &RangeInclusive<u64>,
    r2: &RangeInclusive<u64>,
) -> Option<RangeInclusive<u64>> {
    // https://doc.rust-lang.org/src/core/slice/mod.rs.html#5312
    // with a twist - join when the ranges are neighboring (end of first + 1 = start of second)
    let overlapping = *r1.start() <= (*r2.end() + 1) && *r2.start() <= (*r1.end() + 1);

    if overlapping {
        Some(RangeInclusive::new(
            std::cmp::min(*r1.start(), *r2.start()),
            std::cmp::max(*r1.end(), *r2.end()),
        ))
    } else {
        None
    }
}

impl DisjointRanges {
    fn insert(&mut self, range: RangeInclusive<u64>) {
        if self.ranges.is_empty() {
            self.ranges.push(range);
            return;
        }

        let insert_index = 0;

        // TODO: find insert point

        // TODO: insert

        // TODO: join neighboring ranges
    }

    fn contains(&self, num: u64) -> bool {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::{DisjointRanges, ParsedInput, maybe_join_ranges, parse_input};

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

    #[test]
    fn test_maybe_join_ranges() {
        assert_eq!(maybe_join_ranges(&(5..=6), &(6..=8)), Some(5..=8));
        assert_eq!(maybe_join_ranges(&(6..=8), &(5..=6)), Some(5..=8));
        assert_eq!(maybe_join_ranges(&(4..=4), &(6..=8)), None);
        assert_eq!(maybe_join_ranges(&(6..=8), &(4..=4)), None);
        assert_eq!(maybe_join_ranges(&(6..=8), &(7..=7)), Some(6..=8));
        assert_eq!(maybe_join_ranges(&(7..=7), &(6..=8)), Some(6..=8));

        // Join even if there is a difference of 1
        assert_eq!(maybe_join_ranges(&(6..=8), &(9..=10)), Some(6..=10));
        assert_eq!(maybe_join_ranges(&(9..=10), &(6..=8)), Some(6..=10));
    }

    #[test]
    fn test_disjoint_ranges_join() {
        let mut ranges = DisjointRanges::default();

        ranges.insert(3..=5);
        ranges.insert(5..=6);
        ranges.insert(2..=5);

        assert_eq!(ranges.ranges, vec![2..=6]);
    }

    #[test]
    fn test_disjoint_ranges_join_two_neighbors() {
        let mut ranges = DisjointRanges::default();

        ranges.insert(2..=3);
        ranges.insert(5..=6);

        // Fill in the middle range
        ranges.insert(4..=5);

        assert_eq!(ranges.ranges, vec![2..=6]);
    }

    #[test]
    fn test_disjoint_ranges_join_multiple() {
        let mut ranges = DisjointRanges::default();

        ranges.insert(2..=3);
        ranges.insert(5..=6);
        ranges.insert(8..=9);

        // Overlapping range
        ranges.insert(1..=10);

        assert_eq!(ranges.ranges, vec![1..=10]);
    }
}
