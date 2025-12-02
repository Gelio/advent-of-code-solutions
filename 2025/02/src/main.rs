use std::{
    io::stdin,
    iter::{self},
};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Cannot read input");

    println!("Part 1: {0}", part1(input.trim()));
}

fn part1(input: &str) -> u64 {
    let pairs = input.split(',').map(|pair_str| {
        parse_pair(pair_str)
            .map_err(|err| format!("Could not parse pair {pair_str}: {err:?}"))
            .unwrap()
    });

    pairs
        .map(|(min, max)| invalid_ids(min, max).into_iter().sum::<u64>())
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

fn invalid_ids(min: u64, max: u64) -> Vec<u64> {
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

#[cfg(test)]
mod tests {
    use crate::{invalid_ids, parse_pair, part1};

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair("1-2"), Ok((1, 2)));
        assert_eq!(parse_pair("1-80"), Ok((1, 80)));

        matches!(parse_pair("1-80-"), Err(_));
        matches!(parse_pair("180-"), Err(_));
        matches!(parse_pair("5"), Err(_));
    }

    #[test]
    fn test_count_invalid_ids() {
        assert_eq!(invalid_ids(11, 22), vec![11, 22]);
        assert_eq!(invalid_ids(95, 115), vec![99]);
        assert_eq!(invalid_ids(998, 1012), vec![1010]);
        assert_eq!(invalid_ids(1188511880, 1188511890), vec![1188511885]);
        assert_eq!(invalid_ids(1698522, 1698528), vec![]);
        assert_eq!(invalid_ids(446443, 446449), vec![446446]);
        assert_eq!(invalid_ids(38593856, 38593862), vec![38593859]);
    }

    #[test]
    fn test_part1_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part1(input), 1227775554);
    }
}
