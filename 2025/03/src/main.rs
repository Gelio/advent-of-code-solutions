use std::io::stdin;

fn main() {
    let input: Vec<Vec<u32>> = stdin()
        .lines()
        .map(|line| line.expect("Cannot get stdin line"))
        .map(|line| parse_digits_row(&line).expect("Cannot parse line as digits"))
        .collect();

    println!("Part 1: {0}", solve_part1(&input));
    println!("Part 2: {0}", solve_part2(&input));
}

fn solve_part1(input: &Vec<Vec<u32>>) -> u32 {
    input.iter().map(find_highest_two_batteries_in_row).sum()
}

fn solve_part2(input: &Vec<Vec<u32>>) -> u64 {
    input.iter().map(find_highest_twelve_batteries_in_row).sum()
}

// Part 1, just 2 batteries
fn find_highest_two_batteries_in_row(row: &Vec<u32>) -> u32 {
    let mut highest_digit_till_end: Vec<u32> = vec![0; row.len()];
    highest_digit_till_end[row.len() - 1] = row[row.len() - 1];

    let mut max_joltage: u32 = 0;

    for i in (0..=(row.len() - 2)).rev() {
        let current_digit = row[i];
        let highest_digit_after_current_digit = highest_digit_till_end[i + 1];
        highest_digit_till_end[i] = current_digit.max(highest_digit_after_current_digit);

        let joltage = current_digit * 10 + highest_digit_after_current_digit;
        max_joltage = max_joltage.max(joltage);
    }

    max_joltage
}

// Part 2, use 12 batteries
fn find_highest_twelve_batteries_in_row(row: &Vec<u32>) -> u64 {
    let batteries = 12;

    let mut highest_digits = Vec::from(&row[(row.len() - batteries)..]);

    for i in (0..=(row.len() - batteries - 1)).rev() {
        let current_digit = row[i];
        let leading_highest_digit = highest_digits[0];

        if current_digit < leading_highest_digit {
            continue;
        }

        let mut lowest_digit_index = 0;
        for index in 1..highest_digits.len() {
            if highest_digits[index] < highest_digits[lowest_digit_index] {
                lowest_digit_index = index;
            }
        }

        let digit_to_drop = highest_digits[lowest_digit_index];
        if digit_to_drop == current_digit {
            // NOTE: dropping and inserting the same digit will not change the value.
            // This is a special case when all 12 digits are the same.
            // Not having this `continue` shouldn't change the result, though.
            continue;
        }

        highest_digits.remove(lowest_digit_index);
        highest_digits.splice(0..0, [current_digit]);
    }

    digits_to_number(&highest_digits)
}

fn digits_to_number(digits: &[u32]) -> u64 {
    digits.iter().enumerate().fold(0, |value, (index, digit)| {
        value + u64::from(*digit) * (10 as u64).pow((digits.len() - 1 - index) as u32)
    })
}

fn parse_digits_row(input: &str) -> Result<Vec<u32>, String> {
    input
        .chars()
        .map(|char| {
            char.to_digit(10)
                .ok_or_else(|| format!("Character \"{char}\" is not a valid digit"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{
        digits_to_number, find_highest_twelve_batteries_in_row, find_highest_two_batteries_in_row,
        parse_digits_row, solve_part1, solve_part2,
    };

    #[test]
    fn test_parse_digits_row() {
        assert_eq!(
            parse_digits_row("1234567890"),
            Ok(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0])
        );
        assert_eq!(parse_digits_row("1234"), Ok(vec![1, 2, 3, 4]));
        assert_eq!(
            parse_digits_row("x234"),
            Err("Character \"x\" is not a valid digit".to_string())
        );
    }

    #[test]
    fn test_find_highest_two_batteries_in_row() {
        struct TestCase {
            input: &'static str,
            expected_result: u32,
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                input: "987654321111111",
                expected_result: 98,
            },
            TestCase {
                input: "811111111111119",
                expected_result: 89,
            },
            TestCase {
                input: "234234234234278",
                expected_result: 78,
            },
            TestCase {
                input: "818181911112111",
                expected_result: 92,
            },
        ];

        for test_case in test_cases {
            let digits = parse_digits_row(test_case.input).expect("input should be valid digits");
            assert_eq!(
                find_highest_two_batteries_in_row(&digits),
                test_case.expected_result
            );
        }
    }

    #[test]
    fn test_solve_part1() {
        let input: Vec<Vec<u32>> = "987654321111111
811111111111119
234234234234278
818181911112111"
            .lines()
            .map(|line| parse_digits_row(&line).expect("Cannot parse line as digits"))
            .collect();

        assert_eq!(solve_part1(&input), 357);
    }

    #[test]
    fn test_digits_to_number() {
        assert_eq!(digits_to_number(&[1, 2, 3]), 123);
        assert_eq!(digits_to_number(&[1]), 1);
        assert_eq!(digits_to_number(&[9, 0, 5, 8]), 9058);
    }

    #[test]
    fn test_find_highest_twelve_batteries_in_row() {
        struct TestCase {
            input: &'static str,
            expected_result: u64,
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                input: "987654321111111",
                expected_result: 987654321111,
            },
            TestCase {
                input: "811111111111119",
                expected_result: 811111111119,
            },
            TestCase {
                input: "234234234234278",
                expected_result: 434234234278,
            },
            TestCase {
                input: "818181911112111",
                expected_result: 888911112111,
            },
        ];

        for test_case in test_cases {
            let digits = parse_digits_row(test_case.input).expect("input should be valid digits");
            assert_eq!(
                find_highest_twelve_batteries_in_row(&digits),
                test_case.expected_result
            );
        }
    }

    #[test]
    fn test_solve_part2() {
        let input: Vec<Vec<u32>> = "987654321111111
811111111111119
234234234234278
818181911112111"
            .lines()
            .map(|line| parse_digits_row(&line).expect("Cannot parse line as digits"))
            .collect();

        assert_eq!(solve_part2(&input), 3121910778619);
    }
}
