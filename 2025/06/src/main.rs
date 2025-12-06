use std::{io::stdin, str::FromStr};

fn main() {
    let parsed_input = parse_input(
        stdin()
            .lines()
            .map(|line| line.expect("lines should be valid")),
    )
    .expect("input should be valid");

    println!("Part 1: {}", solve_part1(&parsed_input));
}

fn solve_part1(input: &ParsedInput) -> u64 {
    let mut result = 0;

    for (problem_index, operation) in input.operations_row.iter().enumerate() {
        let problem_result = input
            .number_rows
            .iter()
            .map(|row| row[problem_index])
            .reduce(|problem_result, number| match operation {
                Operation::Add => problem_result + number,
                Operation::Multiply => problem_result * number,
            })
            .expect("rows should not be empty");

        result += problem_result;
    }

    result
}

#[derive(Debug, Default, PartialEq, Eq)]
struct ParsedInput {
    number_rows: Vec<Vec<u64>>,
    operations_row: Vec<Operation>,
}

impl ParsedInput {
    fn verify(&self) -> Result<(), String> {
        if self.number_rows.is_empty() {
            return Err("no number rows".to_string());
        }

        let numbers_per_row = self.number_rows[0].len();

        for row in self.number_rows.iter() {
            if row.len() != numbers_per_row {
                return Err(format!(
                    "mismatched numbers per row, expected {numbers_per_row}, got {}",
                    row.len()
                ));
            }
        }

        if self.operations_row.len() != numbers_per_row {
            return Err(format!(
                "mismatched operations row, expected {numbers_per_row} operations, got {}",
                self.operations_row.len()
            ));
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Add,
    Multiply,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Operation::Multiply),
            "+" => Ok(Operation::Add),
            _ => Err(format!("unsupported operation \"{s}\"")),
        }
    }
}

fn parse_input<S: AsRef<str>>(lines: impl Iterator<Item = S>) -> Result<ParsedInput, String> {
    let mut parsed_input = ParsedInput::default();

    let mut parsed_operations = false;

    for line in lines {
        let mut words = line.as_ref().split_ascii_whitespace().peekable();

        let Some(first_word) = words.peek() else {
            return Err("empty line".to_string());
        };

        if parsed_operations {
            return Err("unexpected line after parsing operations".to_string());
        }

        if Operation::from_str(first_word).is_ok() {
            // Operations row
            parsed_input.operations_row =
                words.map(|word| word.parse()).collect::<Result<_, _>>()?;
            parsed_operations = true;
        } else {
            // Numbers row
            let parsed_numbers: Vec<u64> = words
                .map(|word| word.parse())
                .collect::<Result<_, _>>()
                .map_err(|err| format!("could not parse number {err:?}"))?;
            parsed_input.number_rows.push(parsed_numbers);
        }
    }

    parsed_input.verify().map(|_| parsed_input)
}

#[cfg(test)]
mod tests {
    use crate::{Operation, ParsedInput, parse_input, solve_part1};

    #[test]
    fn test_parse_input() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
        let parsed_input = parse_input(input.lines());

        assert_eq!(
            parsed_input,
            Ok(ParsedInput {
                number_rows: vec![
                    vec![123, 328, 51, 64],
                    vec![45, 64, 387, 23],
                    vec![6, 98, 215, 314]
                ],
                operations_row: vec![
                    Operation::Multiply,
                    Operation::Add,
                    Operation::Multiply,
                    Operation::Add
                ],
            })
        );
    }

    #[test]
    fn test_solve_part1() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
        let parsed_input = parse_input(input.lines()).expect("input should be valid");

        assert_eq!(solve_part1(&parsed_input), 4277556);
    }
}
