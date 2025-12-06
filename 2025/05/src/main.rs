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
    let ranges = DisjointRanges::from_iter(parsed_input.ranges.iter().cloned());

    println!(
        "Part 1: {}",
        solve_part1(&ranges, parsed_input.numbers.iter().copied())
    );
    println!("Part 2: {}", solve_part2(&ranges));
}

fn solve_part1(ranges: &DisjointRanges, numbers: impl Iterator<Item = u64>) -> u32 {
    let mut result = 0;

    for num in numbers {
        if ranges.contains(num) {
            result += 1;
        }
    }

    result
}

fn solve_part2(ranges: &DisjointRanges) -> u64 {
    ranges
        .iter()
        .map(|range| *range.end() - *range.start() + 1)
        .sum()
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
        self.insert_only(range);

        #[cfg(debug_assertions)]
        self.ensure_valid();
    }

    fn insert_only(&mut self, mut range: RangeInclusive<u64>) {
        let mut index = 0;
        while index < self.ranges.len() {
            let other_range = &self.ranges[index];

            if *range.start() > *other_range.end() + 1 {
                index += 1;
                continue;
            }

            if let Some(joined_range) = maybe_join_ranges(&range, other_range) {
                range = joined_range;
                self.ranges.remove(index);
                // NOTE: do not increment the index here.
                // Removing the range at `index` will shift all elements left,
                // so `index` will now point to the next range.
            } else {
                // NOTE: range < other_range, so insert `range` before `other_range`
                self.ranges.insert(index, range);
                return;
            }
        }

        self.ranges.push(range);
    }

    fn contains(&self, num: u64) -> bool {
        for range in self.ranges.iter() {
            if num < *range.start() {
                return false;
            }

            if num <= *range.end() {
                return true;
            }
        }

        false
    }

    fn ensure_valid(&self) {
        for range_pair in self.ranges.windows(2) {
            let [r1, r2] = range_pair else {
                panic!("window did not contain two ranges")
            };

            assert!(r1.start() < r2.start(), "{r1:?} must start before {r2:?}");

            assert!(
                r1.end() < r2.start(),
                "{r1:?} must end before {r2:?} starts"
            );

            let distance = r2.start() - r1.end();
            assert!(
                distance > 1,
                "distance between {r1:?} and {r2:?} must be greater than 1, found {distance}"
            );
        }
    }

    fn iter(&self) -> impl Iterator<Item = &RangeInclusive<u64>> {
        self.ranges.iter()
    }
}

impl FromIterator<RangeInclusive<u64>> for DisjointRanges {
    fn from_iter<T: IntoIterator<Item = RangeInclusive<u64>>>(iter: T) -> Self {
        let mut r = Self::default();

        for v in iter {
            r.insert(v);
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        DisjointRanges, ParsedInput, maybe_join_ranges, parse_input, solve_part1, solve_part2,
    };

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
    fn test_solve_part1() {
        let input = ParsedInput {
            ranges: vec![3..=5, 10..=14, 16..=20, 12..=18],
            numbers: vec![1, 5, 8, 11, 17, 32],
        };
        let ranges = DisjointRanges::from_iter(input.ranges.iter().cloned());

        assert_eq!(solve_part1(&ranges, input.numbers.iter().copied()), 3);
    }

    #[test]
    fn test_solve_part2() {
        let ranges = DisjointRanges::from_iter(vec![3..=5, 10..=14, 16..=20, 12..=18]);

        assert_eq!(solve_part2(&ranges), 14);
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

    #[test]
    fn test_disjoint_ranges_contains() {
        let ranges = DisjointRanges::from_iter(vec![1..=5, 7..=10]);

        assert_eq!(ranges.contains(0), false);
        assert_eq!(ranges.contains(1), true);
        assert_eq!(ranges.contains(2), true);
        assert_eq!(ranges.contains(3), true);
        assert_eq!(ranges.contains(4), true);
        assert_eq!(ranges.contains(5), true);
        assert_eq!(ranges.contains(6), false);
        assert_eq!(ranges.contains(7), true);
        assert_eq!(ranges.contains(8), true);
        assert_eq!(ranges.contains(9), true);
        assert_eq!(ranges.contains(10), true);
        assert_eq!(ranges.contains(11), false);
    }
}
