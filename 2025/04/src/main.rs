use std::{collections::HashSet, io::stdin};

fn main() {
    let input = parse_input(
        stdin()
            .lines()
            .map(|line| line.expect("should be valid line")),
    )
    .expect("should parse input");

    let (initial_accessible_rolls, surrounding_rolls_count) = solve_part1(&input);
    println!("Part 1: {0}", initial_accessible_rolls.len());
    println!(
        "Part 2: {0}",
        solve_part2(input, initial_accessible_rolls, surrounding_rolls_count)
    );
}

type TileMap = Vec<Vec<Tile>>;
type SurroundingRollsCountMap = Vec<Vec<u32>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

fn solve_part1(input: &TileMap) -> (Vec<Position>, SurroundingRollsCountMap) {
    let mut accessible_rolls: Vec<Position> = Vec::new();
    let mut surrounding_rolls_count: SurroundingRollsCountMap =
        vec![vec![0; input[0].len()]; input.len()];

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == Tile::Empty {
                // We only care about counting occupied tiles (rolls of paper)
                continue;
            }

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
                        surrounding_rolls_count[y][x] += 1;
                    }
                }
            }

            if surrounding_rolls_count[y][x] < 4 {
                accessible_rolls.push(Position { x, y })
            }
        }
    }

    (accessible_rolls, surrounding_rolls_count)
}

fn solve_part2(
    mut input: TileMap,
    initial_accessible_rolls: Vec<Position>,
    mut surrounding_rolls_count: SurroundingRollsCountMap,
) -> u32 {
    let mut rolls_removed = 0;

    let mut accessible_rolls: HashSet<Position> =
        HashSet::from_iter(initial_accessible_rolls.into_iter());

    while let Some(accessible_roll_position) = accessible_rolls.iter().next().cloned() {
        accessible_rolls.remove(&accessible_roll_position);
        rolls_removed += 1;
        let Position { x, y } = accessible_roll_position;
        input[y][x] = Tile::Empty;

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

                if input[row_index][column_index] == Tile::Empty {
                    continue;
                }

                surrounding_rolls_count[row_index][column_index] -= 1;
                let neighbors_left = surrounding_rolls_count[row_index][column_index];

                if neighbors_left < 4 {
                    accessible_rolls.insert(Position {
                        x: column_index,
                        y: row_index,
                    });
                }
            }
        }
    }

    rolls_removed
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
    use crate::{parse_input, solve_part1, solve_part2};

    #[test]
    fn test_example_input() {
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

        let (initial_accessible_rolls, surrounding_rolls_count) = solve_part1(&example_input);
        assert_eq!(initial_accessible_rolls.len(), 13);
        assert_eq!(
            solve_part2(
                example_input,
                initial_accessible_rolls,
                surrounding_rolls_count
            ),
            43
        );
    }
}
