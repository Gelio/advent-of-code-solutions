use std::{collections::HashSet, io::stdin};

fn main() {
    let input = parse_input(
        stdin()
            .lines()
            .map(|line| line.expect("line should be valid")),
    )
    .expect("input should be valid");

    println!("Part 1: {}", solve_part1(&input));
}

fn solve_part1(input: &ParsedInput) -> u32 {
    let mut beam_indexes: HashSet<usize> = HashSet::new();
    beam_indexes.insert(input.start_index);

    let mut beam_split_times = 0;

    for line in input.lines.iter() {
        for index in line.splitter_indexes.iter() {
            if beam_indexes.contains(index) {
                beam_indexes.remove(index);
                beam_indexes.insert(index - 1);
                // TODO: delay this call to avoid false-positives,
                // e.g. beam_indexes = [1], splitter_indexes = [1, 2],
                // expected_result = [1, 2],
                // actual_result = [1, 2, 3] (because index 2 will also be split)
                beam_indexes.insert(index + 1);

                beam_split_times += 1;
            }
        }
    }

    beam_split_times
}

#[derive(Debug, PartialEq, Eq, Default)]
struct ParsedInputLine {
    splitter_indexes: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsedInput {
    start_index: usize,
    lines: Vec<ParsedInputLine>,
}

fn parse_input<Str, LinesIter>(mut lines: LinesIter) -> Result<ParsedInput, String>
where
    Str: AsRef<str>,
    LinesIter: Iterator<Item = Str>,
{
    let start_index = lines
        .next()
        .ok_or_else(|| format!("input is empty"))?
        .as_ref()
        .char_indices()
        .find_map(|(index, c)| (c == 'S').then_some(index))
        .ok_or_else(|| format!("the first line does not contain the starting position"))?;

    let parsed_lines: Vec<ParsedInputLine> = lines
        .map(|line| {
            let line = line.as_ref();
            let mut parsed_line = ParsedInputLine::default();

            for (index, c) in line.char_indices() {
                if c == '^' {
                    parsed_line.splitter_indexes.push(index);
                }
            }

            parsed_line
        })
        .collect();

    Ok(ParsedInput {
        start_index,
        lines: parsed_lines,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        let parsed_input = parse_input(input.lines()).expect("input should be valid");
        assert_eq!(solve_part1(&parsed_input), 21);
    }
}
