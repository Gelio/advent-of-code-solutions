use std::{
    collections::HashSet,
    io::stdin,
    iter::{self},
};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Cannot read input");

    let pairs = parse_input(input.trim());

    println!("Part 1: {0}", part1(&pairs));
    println!("Part 2: {0}", part2(&pairs));
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .map(|pair_str| {
            parse_pair(pair_str)
                .map_err(|err| format!("Could not parse pair {pair_str}: {err:?}"))
                .unwrap()
        })
        .collect::<Vec<_>>()
}

fn part1(pairs: &Vec<(u64, u64)>) -> u64 {
    pairs
        .iter()
        .map(|(min, max)| invalid_ids_halves(*min, *max).into_iter().sum::<u64>())
        .sum()
}

fn part2(pairs: &Vec<(u64, u64)>) -> u64 {
    pairs
        .iter()
        .map(|(min, max)| invalid_ids_any_length(*min, *max).into_iter().sum::<u64>())
        .sum()
}

fn parse_pair(pair_str: &str) -> Result<(u64, u64), String> {
    let parts_result = pair_str
        .split('-')
        .map(|part| part.parse())
        .collect::<Result<Vec<u64>, _>>();

    let parts = parts_result.map_err(|error| format!("Cannot parse some number {error:?}"))?;
    if parts.len() != 2 {
        return Err(format!("{pair_str} does not form a valid pair"));
    }

    Ok((parts[0], parts[1]))
}

// Find invalid IDs for part 1
fn invalid_ids_halves(min: u64, max: u64) -> Vec<u64> {
    let min_str = min.to_string();

    // Digits from the higher half of the number.
    // Example:
    // For 101054 => 101
    // For 12348580 => 1234
    //
    // For numbers with odd length, there won't be any invalid IDs for numbers of this length
    // (because invalid IDs have even length, since they comprise of two halves).
    // Thus, start with the next number that has 1 magnitude higher.
    // Example:
    // 123 => 1 => 10
    // 85894 => 85 => 100
    // 7580485 => 758 => 1000
    let higher_half = if min_str.len() % 2 == 0 {
        String::from(&min_str[0..(min_str.len() / 2)])
    } else {
        iter::once('1')
            .chain(iter::repeat('0').take(min_str.len() / 2))
            .collect()
    };
    let mut higher_half: u32 = higher_half.parse().expect("Could not parse higher half");

    let mut invalid_ids: Vec<u64> = Vec::new();

    loop {
        let id = higher_half.to_string().repeat(2);
        let id: u64 = id.parse().expect("Could not parse ID");

        higher_half += 1;

        if id < min {
            // This can happen for range (565653, 565659)
            continue;
        }

        if id > max {
            break;
        }
        invalid_ids.push(id);
    }

    invalid_ids
}

// NOTE: this function did not work correctly in the actual full input for day 2 for part 2 :/
fn are_halves_repeated(num: &str) -> bool {
    if num.len() == 1 {
        return false;
    }

    let middle_index = num.len() / 2;
    let higher_half = if num.len() % 2 == 0 {
        &num[0..middle_index]
    } else {
        // NOTE: for numbers of odd length (e.g. 55555), the middle digit should appear in both
        // lower and higher halves
        &num[0..=middle_index]
    };
    let lower_half = &num[middle_index..];

    higher_half == lower_half
}

// Find invalid IDs for part 2
fn invalid_ids_any_length(min: u64, max: u64) -> Vec<u64> {
    let max_str = max.to_string();

    // NOTE: initially I was using a Vec and then used `are_halves_repeated` to detect duplicates,
    // but that rejected too many numbers.
    // let mut invalid_ids: Vec<u64> = Vec::new();
    // There is some bug in `are_halves_repeated` and I can't find it. Thus, I'm using a HashSet.
    // It's fast enough (both parts execute in 0.03s in release mode on my Mac).
    let mut invalid_ids: HashSet<u64> = HashSet::new();

    for sequence_length in 1..=(max_str.len() / 2) {
        let mut repeated_digits: u64 = iter::once('1')
            .chain(iter::repeat('0').take(sequence_length - 1))
            .collect::<String>()
            .parse()
            .expect("Cannot parse starting repeated digits");

        'increment_digits: loop {
            #[cfg(test)]
            dbg!(repeated_digits);

            // if are_halves_repeated(&repeated_digits.to_string()) {
            //     repeated_digits += 1;
            //     continue;
            // }
            let repeated_digits_str = repeated_digits.to_string();
            if repeated_digits_str.len() != sequence_length {
                break 'increment_digits;
            }

            // let starting_sequence_repeats = min_str.len() / sequence_length;
            let starting_sequence_repeats = 2;

            'sequence_lengths: for sequence_repeats in
                starting_sequence_repeats..=(max_str.len() / sequence_length)
            {
                let id = repeated_digits_str.repeat(sequence_repeats);
                let id: u64 = id.parse().expect("Could not parse ID");

                #[cfg(test)]
                dbg!(id);

                if id < min {
                    continue;
                }

                if id > max {
                    if sequence_repeats == starting_sequence_repeats {
                        break 'increment_digits;
                    } else {
                        break 'sequence_lengths;
                    }
                }

                #[cfg(test)]
                println!("matches");
                // invalid_ids.push(id);
                invalid_ids.insert(id);
            }

            repeated_digits += 1;
        }
    }

    let mut invalid_ids: Vec<_> = invalid_ids.into_iter().collect();

    #[cfg(test)]
    invalid_ids.sort();

    invalid_ids
}

#[cfg(test)]
mod tests {
    use crate::{
        are_halves_repeated, invalid_ids_any_length, invalid_ids_halves, parse_input, parse_pair,
        part1, part2,
    };

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair("1-2"), Ok((1, 2)));
        assert_eq!(parse_pair("1-80"), Ok((1, 80)));

        matches!(parse_pair("1-80-"), Err(_));
        matches!(parse_pair("180-"), Err(_));
        matches!(parse_pair("5"), Err(_));
    }

    #[test]
    fn test_count_invalid_ids_halves() {
        assert_eq!(invalid_ids_halves(11, 22), vec![11, 22]);
        assert_eq!(invalid_ids_halves(95, 115), vec![99]);
        assert_eq!(invalid_ids_halves(998, 1012), vec![1010]);
        assert_eq!(invalid_ids_halves(1188511880, 1188511890), vec![1188511885]);
        assert_eq!(invalid_ids_halves(1698522, 1698528), vec![]);
        assert_eq!(invalid_ids_halves(446443, 446449), vec![446446]);
        assert_eq!(invalid_ids_halves(38593856, 38593862), vec![38593859]);
    }

    #[test]
    fn test_are_halves_repeated() {
        assert_eq!(are_halves_repeated("1212"), true);
        assert_eq!(are_halves_repeated("555555"), true);
        assert_eq!(are_halves_repeated("55555"), true);
        assert_eq!(are_halves_repeated("123123"), true);
        assert_eq!(are_halves_repeated("123124"), false);
        assert_eq!(are_halves_repeated("1221"), false);
        assert_eq!(are_halves_repeated("121"), false);
    }

    #[test]
    fn test_count_invalid_ids_any_length() {
        assert_eq!(invalid_ids_any_length(11, 22), vec![11, 22]);
        assert_eq!(invalid_ids_any_length(95, 115), vec![99, 111]);
        assert_eq!(invalid_ids_any_length(998, 1012), vec![999, 1010]);
        assert_eq!(
            invalid_ids_any_length(1188511880, 1188511890),
            vec![1188511885]
        );
        assert_eq!(invalid_ids_any_length(222220, 222224), vec![222222]);
        assert_eq!(invalid_ids_any_length(1698522, 1698528), vec![]);
        assert_eq!(invalid_ids_any_length(446443, 446449), vec![446446]);
        assert_eq!(invalid_ids_any_length(38593856, 38593862), vec![38593859]);
        assert_eq!(invalid_ids_any_length(565653, 565659), vec![565656]);
        assert_eq!(
            invalid_ids_any_length(824824821, 824824827),
            vec![824824824]
        );
        assert_eq!(
            invalid_ids_any_length(2121212118, 2121212124),
            vec![2121212121]
        );
    }

    const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1_example() {
        let pairs = parse_input(EXAMPLE_INPUT);
        assert_eq!(part1(&pairs), 1227775554);
    }

    #[test]
    fn test_part2_example() {
        let pairs = parse_input(EXAMPLE_INPUT);
        assert_eq!(part2(&pairs), 4174379265);
    }
}
