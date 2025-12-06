use crate::operation::Operation;

pub fn solve(input: &str) -> u64 {
    parse_problems(input)
        .expect("input should be valid")
        .iter()
        .map(Problem::solve)
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Problem {
    numbers: Vec<u64>,
    operation: Operation,
}

impl Problem {
    fn solve(&self) -> u64 {
        self.numbers
            .iter()
            .copied()
            .reduce(|problem_result, number| match self.operation {
                Operation::Add => problem_result + number,
                Operation::Multiply => problem_result * number,
            })
            .expect("problem numbers should not be empty")
    }
}

fn parse_problems(input: &str) -> Result<Vec<Problem>, String> {
    let columns = parse_columns(input)?;

    columns
        .split(|column| *column == Column::Empty)
        .map(|problem_columns| {
            let mut numbers: Vec<u64> = Vec::new();
            let mut operation: Option<Operation> = None;

            for column in problem_columns {
                match column {
                    Column::Number(number) => {
                        numbers.push(*number);
                    }
                    Column::NumberAndOperation {
                        number,
                        operation: op,
                    } => {
                        numbers.push(*number);
                        operation = Some(*op);
                    }
                    Column::Empty => unreachable!("the columns were split by empty columns"),
                }
            }

            let operation =
                operation.ok_or_else(|| "operation was not found in problem".to_string())?;

            if numbers.is_empty() {
                Err("numbers are empty in problem".to_string())
            } else {
                Ok(Problem { numbers, operation })
            }
        })
        .collect()
}

fn parse_columns(input: &str) -> Result<Vec<Column>, String> {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let columns_count = chars
        .first()
        .map(|line| line.len())
        .expect("input should not be empty");

    (0..columns_count)
        .map(|column_index| {
            Column::from_iter(ColumnIterator::new(&chars, column_index))
                .map_err(|err| format!("could not parse column index {column_index}: {err:?}"))
        })
        .collect::<Result<_, _>>()
}

#[derive(Debug, PartialEq, Eq)]
enum Column {
    Number(u64),
    NumberAndOperation { number: u64, operation: Operation },
    Empty,
}

impl Column {
    fn from_iter(value: impl Iterator<Item = char>) -> Result<Self, String> {
        let mut column = Column::Empty;

        for c in value {
            if let Ok(operation) = Operation::try_from(c) {
                column = match column {
                    Column::Number(num) => Column::NumberAndOperation {
                        number: num,
                        operation,
                    },
                    Column::NumberAndOperation { .. } => {
                        return Err(format!(
                            "operation appeared twice in column {column:?}, received {operation:?}"
                        ));
                    }
                    Column::Empty => {
                        return Err(format!(
                            "received operation {operation:?} without any preceding digits"
                        ));
                    }
                };
                continue;
            }

            if let Some(digit) = c.to_digit(10) {
                column = match column {
                    Column::Number(number) => Column::Number(number * 10 + u64::from(digit)),
                    Column::NumberAndOperation { .. } => {
                        return Err(format!(
                            "unexpected digit {digit} after operation in column {column:?}"
                        ));
                    }
                    Column::Empty => Column::Number(u64::from(digit)),
                };
                continue;
            }

            if c != ' ' {
                return Err(format!("unexpected character {c}"));
            }
        }

        Ok(column)
    }
}

struct ColumnIterator<'a> {
    chars: &'a Vec<Vec<char>>,
    column_index: usize,
    row_index: usize,
}

impl<'a> ColumnIterator<'a> {
    fn new(chars: &'a Vec<Vec<char>>, column_index: usize) -> Self {
        Self {
            chars,
            column_index,
            row_index: 0,
        }
    }
}

impl<'a> Iterator for ColumnIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.chars.get(self.row_index)?;
        self.row_index += 1;

        Some(
            *row.get(self.column_index)
                .expect("rows should be of equal length"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operation::Operation;

    #[test]
    fn test_solve() {
        // NOTE: \x20 for trailing space
        let input = "123 328  51 64\x20
 45 64  387 23\x20
  6 98  215 314
*   +   *   +  ";

        assert_eq!(solve(&input), 3263827);
    }

    #[test]
    fn test_parse_problems() {
        // NOTE: \x20 for trailing space
        let input = "123 328  51 64\x20
 45 64  387 23\x20
  6 98  215 314
*   +   *   +  ";

        assert_eq!(
            parse_problems(&input),
            Ok(vec![
                Problem {
                    numbers: vec![1, 24, 356],
                    operation: Operation::Multiply
                },
                Problem {
                    numbers: vec![369, 248, 8],
                    operation: Operation::Add
                },
                Problem {
                    numbers: vec![32, 581, 175],
                    operation: Operation::Multiply
                },
                Problem {
                    numbers: vec![623, 431, 4],
                    operation: Operation::Add
                },
            ])
        );
    }

    #[test]
    fn test_parse_columns() {
        // NOTE: \x20 for trailing space
        let input = "123 328
 45 64\x20
  6 98\x20
*   +  ";

        assert_eq!(
            parse_columns(input),
            Ok(vec![
                Column::NumberAndOperation {
                    number: 1,
                    operation: Operation::Multiply
                },
                Column::Number(24),
                Column::Number(356),
                Column::Empty,
                Column::NumberAndOperation {
                    number: 369,
                    operation: Operation::Add
                },
                Column::Number(248),
                Column::Number(8),
            ])
        )
    }

    #[test]
    fn test_column_iterator() {
        // NOTE: \x20 for trailing space
        let input = "123 328  51 64\x20
 45 64  387 23\x20
  6 98  215 314
*   +   *   +  ";

        let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        dbg!(&chars);

        assert_eq!(
            ColumnIterator::new(&chars, 0).collect::<Vec<char>>(),
            vec!['1', ' ', ' ', '*']
        )
    }

    #[test]
    fn test_column_from_iter() {
        assert_eq!(
            Column::from_iter(vec!['1', '2', '3'].into_iter()),
            Ok(Column::Number(123))
        );
        assert_eq!(
            Column::from_iter(vec![' ', '2', ' ', '*'].into_iter()),
            Ok(Column::NumberAndOperation {
                number: 2,
                operation: Operation::Multiply,
            })
        );
        assert_eq!(
            Column::from_iter(vec!['1', '2', ' ', '+'].into_iter()),
            Ok(Column::NumberAndOperation {
                number: 12,
                operation: Operation::Add,
            })
        );
        assert_eq!(
            Column::from_iter(vec![' ', ' ', ' '].into_iter()),
            Ok(Column::Empty)
        );

        matches!(Column::from_iter(vec![' ', ' ', '*'].into_iter()), Err(_));
        matches!(
            Column::from_iter(vec!['2', ' ', '*', '*'].into_iter()),
            Err(_)
        );
        matches!(
            Column::from_iter(vec!['2', ' ', '*', '2'].into_iter()),
            Err(_)
        );
    }
}
