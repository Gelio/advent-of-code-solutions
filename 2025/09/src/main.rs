use std::{collections::BinaryHeap, fmt::Display, io::stdin, ops::Sub, str::FromStr};

fn main() {
    let points = parse_input(
        stdin()
            .lines()
            .map(|line| line.expect("lines should be valid")),
    );

    println!("Part 1: {}", solve_part1(&points));
    println!("Part 2: {}", solve_part2(&points));
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

fn solve_part1(points: &Vec<Position>) -> u128 {
    let mut max_area: Option<u128> = None;

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

fn solve_part2(points: &Vec<Position>) -> u128 {
    let mut rectangles: BinaryHeap<(u128, Position, Position)> = BinaryHeap::new();

    for (i1, p1) in points.iter().enumerate() {
        for p2 in &points[i1 + 1..] {
            rectangles.push((rectangle_area(p1, p2), p1.clone(), p2.clone()))
        }
    }

    'rectangles: while let Some((area, p1, p2)) = rectangles.pop() {
        for segment in points.windows(2) {
            let p3 = &segment[0];
            let p4 = &segment[1];

            if rectangle_intersects_segment((&p1, &p2), (p3, p4)) {
                continue 'rectangles;
            }
        }

        // NOTE: also check the final line
        if rectangle_intersects_segment(
            (&p1, &p2),
            (
                &points.last().expect("points should not be empty"),
                &points[0],
            ),
        ) {
            continue 'rectangles;
        }

        dbg!(&p1, &p2);
        return area;
    }

    unreachable!("some rectangle must be valid");
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Position {
    x: i128,
    y: i128,
}

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<i128> = s
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

impl Sub for &Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Position {
    fn new(x: i128, y: i128) -> Self {
        Self { x, y }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("{},{}", self.x, self.y))
    }
}

fn rectangle_intersects_segment(
    rect_corners: (&Position, &Position),
    segment: (&Position, &Position),
) -> bool {
    let p1 = rect_corners.0;
    let p2 = &Position::new(rect_corners.0.x, rect_corners.1.y);
    let p3 = rect_corners.1;
    let p4 = &Position::new(rect_corners.1.x, rect_corners.0.y);

    let segment = if segment.0.x == segment.1.x {
        (
            &Position::new(
                segment.0.x,
                segment.0.y + 1 * (segment.0.y - segment.1.y).signum(),
            ),
            &Position::new(
                segment.1.x,
                segment.1.y + 1 * (segment.1.y - segment.0.y).signum(),
            ),
        )
    } else {
        (
            &Position::new(
                segment.0.x + 1 * (segment.0.x - segment.1.x).signum(),
                segment.0.y,
            ),
            &Position::new(
                segment.1.x + 1 * (segment.1.x - segment.0.x).signum(),
                segment.1.y,
            ),
        )
    };

    segments_intersect((p1, p2), segment)
        || segments_intersect((p2, p3), segment)
        || segments_intersect((p3, p4), segment)
        || segments_intersect((p4, p1), segment)
}

fn rectangle_area(p1: &Position, p2: &Position) -> u128 {
    let x_size = p1.x.abs_diff(p2.x) + 1;
    let y_size = p1.y.abs_diff(p2.y) + 1;

    x_size * y_size
}

// Intersection of two segments
// https://www.reddit.com/r/algorithms/comments/9moad4/comment/e7gvsjv/
fn cross(p1: &Position, p2: &Position) -> i128 {
    (p1.x * p2.y) as i128 - (p1.y * p2.x) as i128
}

fn orient(p1: &Position, p2: &Position, p3: &Position) -> i128 {
    cross(&(p2 - p1), &(p3 - p1))
}

fn segments_intersect((p1, p2): (&Position, &Position), (p3, p4): (&Position, &Position)) -> bool {
    let oa = orient(p3, p4, p1);
    let ob = orient(p3, p4, p2);
    let oc = orient(p1, p2, p3);
    let od = orient(p1, p2, p4);

    oa * ob < 0 && oc * od < 0
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

    #[test]
    fn test_segments_intersect() {
        assert_eq!(
            segments_intersect(
                (&Position::new(1, 2), &Position::new(1, 8)),
                (&Position::new(0, 5), &Position::new(8, 5))
            ),
            true
        );

        assert_eq!(
            segments_intersect(
                (&Position::new(1, 2), &Position::new(1, 8)),
                (&Position::new(1, 5), &Position::new(3, 5))
            ),
            // NOTE: no intersection if they only touch (intersect at one of the positions)
            false
        );
        assert_eq!(
            segments_intersect(
                (&Position::new(2, 3), &Position::new(7, 3)),
                (&Position::new(7, 3), &Position::new(7, 1))
            ),
            // NOTE: no intersection if they only touch (intersect at one of the positions)
            false
        );

        assert_eq!(
            segments_intersect(
                (&Position::new(1, 2), &Position::new(1, 8)),
                (&Position::new(0, 5), &Position::new(3, 5))
            ),
            true
        );

        assert_eq!(
            segments_intersect(
                (&Position::new(1, 2), &Position::new(1, 8)),
                (&Position::new(1, 5), &Position::new(1, 10)),
            ),
            // NOTE: no intersection if the segments overlap
            false
        );
    }

    #[test]
    fn test_rectangle_intersects_segment() {
        assert_eq!(
            rectangle_intersects_segment(
                (&Position::new(2, 3), &Position::new(9, 7)),
                (&Position::new(2, 5), &Position::new(9, 5))
            ),
            true
        );

        assert_eq!(
            rectangle_intersects_segment(
                (&Position::new(7, 3), &Position::new(9, 5)),
                (&Position::new(2, 5), &Position::new(9, 5))
            ),
            false
        );

        assert_eq!(
            rectangle_intersects_segment(
                (&Position::new(7, 1), &Position::new(11, 7)),
                (&Position::new(2, 5), &Position::new(9, 5))
            ),
            true
        );
        assert_eq!(
            rectangle_intersects_segment(
                (&Position::new(7, 1), &Position::new(11, 7)),
                (&Position::new(9, 7), &Position::new(11, 7))
            ),
            false
        );
        assert_eq!(
            rectangle_intersects_segment(
                (&Position::new(7, 1), &Position::new(11, 7)),
                (&Position::new(2, 3), &Position::new(3, 5))
            ),
            false
        );
    }

    #[test]
    fn test_solve_part2() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        let points = parse_input(input.lines());
        assert_eq!(solve_part2(&points), 24);
    }
}
