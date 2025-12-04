use std::io::stdin;

fn main() {
    let input = parse_input(
        stdin()
            .lines()
            .map(|line| line.expect("should be valid line")),
    )
    .expect("should parse input");

    println!("Part 1: {0}", solve_part1(&input))
}

type TileMap = Vec<Vec<Tile>>;

fn solve_part1(input: &TileMap) -> u32 {
    let mut result = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == Tile::Empty {
                // We only care about counting occupied tiles (rolls of paper)
                continue;
            }

            let mut surrounding_tiles_occupied = 0;

            for y_delta in -1..=1 {
                if y == 0 && y_delta == -1 {
                    continue;
                }

                let row_index = (y as isize + y_delta) as usize;
                if row_index >= input.len() {
                    continue;
                }

                for x_delta in -1..=1 {
                    if y_delta == 0 && x_delta == 0 {
                        // Do not count the tile itself
                        continue;
                    }

                    if x == 0 && x_delta == -1 {
                        continue;
                    }

                    let column_index = (x as isize + x_delta) as usize;
                    if column_index >= input[row_index].len() {
                        continue;
                    }

                    if input[row_index][column_index] == Tile::Occupied {
                        surrounding_tiles_occupied += 1;
                    }
                }
            }

            if surrounding_tiles_occupied < 4 {
                result += 1;
            }
        }
    }

    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Occupied,
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '@' => Ok(Self::Occupied),
            '.' => Ok(Self::Empty),
            c => Err(format!(r#"character "{0}" is not a valid tile"#, c)),
        }
    }
}

fn parse_input<'a>(input: impl Iterator<Item = impl AsRef<str>>) -> Result<TileMap, String> {
    input
        .into_iter()
        .map(|line| {
            line.as_ref()
                .chars()
                .map(|c| c.try_into())
                .collect::<Result<Vec<Tile>, String>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, solve_part1};

    #[test]
    fn test_example_input_part1() {
        let example_input = parse_input(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
                .lines(),
        )
        .expect("should parse");

        assert_eq!(example_input.len(), 10);
        assert_eq!(example_input[0].len(), 10);

        assert_eq!(solve_part1(&example_input), 13);
    }
}
