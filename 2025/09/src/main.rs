use std::{io::stdin, str::FromStr};

fn main() {
    let points = parse_input(
        stdin()
            .lines()
            .map(|line| line.expect("lines should be valid")),
    );

    println!("Part 1: {}", solve_part1(&points));
}

fn parse_input(lines: impl Iterator<Item = impl AsRef<str>>) -> Vec<Position> {
    lines
        .map(|line| {
            line.as_ref()
                .parse()
                .map_err(|err| {
                    format!(
                        "cannot parse line \"{line}\" as position: {err:?}",
                        line = line.as_ref()
                    )
                })
                .expect("input should be valid")
        })
        .collect()
}

fn solve_part1(points: &Vec<Position>) -> u64 {
    let mut max_area: Option<u64> = None;

    for (i1, p1) in points.iter().enumerate() {
        for p2 in &points[i1 + 1..] {
            let current_area = rectangle_area(p1, p2);

            if let Some(acc_max_area) = max_area {
                if current_area > acc_max_area {
                    max_area = Some(current_area);
                }
            } else {
                max_area = Some(current_area);
            }
        }
    }

    max_area.expect("there should be at least 2 points in the list")
}

#[derive(Debug, PartialEq, Eq)]
struct Position {
    x: u64,
    y: u64,
}

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<u64> = s
            .split(',')
            .map(|part| {
                part.parse()
                    .map_err(|err| format!("cannot parse {part} as number: {err:?}"))
            })
            .collect::<Result<_, _>>()?;

        assert_eq!(parts.len(), 2);

        Ok(Self {
            x: parts[0],
            y: parts[1],
        })
    }
}

fn rectangle_area(p1: &Position, p2: &Position) -> u64 {
    let x_size = p1.x.abs_diff(p2.x) + 1;
    let y_size = p1.y.abs_diff(p2.y) + 1;

    x_size * y_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        let points = parse_input(input.lines());
        assert_eq!(solve_part1(&points), 50);
    }
}
