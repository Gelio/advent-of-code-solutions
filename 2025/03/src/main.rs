use std::io::stdin;

fn main() {
    let input: Vec<Vec<u32>> = stdin()
        .lines()
        .map(|line| line.expect("Cannot get stdin line"))
        .map(|line| parse_digits_row(&line).expect("Cannot parse line as digits"))
        .collect();

    println!("Part 1: {0}", solve_part1(&input));
}

fn solve_part1(input: &Vec<Vec<u32>>) -> u32 {
    input.iter().map(find_highest_two_batteries_in_row).sum()
}

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
    use crate::{find_highest_two_batteries_in_row, parse_digits_row, solve_part1};

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
}
