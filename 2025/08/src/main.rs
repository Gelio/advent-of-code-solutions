use std::{collections::HashSet, fmt::Display, io::stdin, str::FromStr};

use aoc_2025_08::union_find::UnionFind;

fn main() {
    let positions = parse_positions(
        stdin()
            .lines()
            .map(|line| line.expect("lines should be valid")),
    );
    let mut connector = ComponentConnector::new(positions);
    println!("Part 1: {}", solve_part1(&mut connector));
}

fn parse_positions(input: impl Iterator<Item = impl AsRef<str>>) -> Vec<Position> {
    input
        .map(|line| line.as_ref().parse().expect("position should be valid"))
        .collect()
}

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    fn distance_squared(&self, other: &Position) -> u64 {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("{},{},{}", self.x, self.y, self.z))
    }
}

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s
            .split(',')
            .map(|n| {
                n.parse()
                    .map_err(|err| format!("cannot parse {n} as number: {err:?}"))
            })
            .collect::<Result<_, _>>()?;

        if parts.len() != 3 {
            return Err(format!("invalid length, expected 3, found {}", parts.len()));
        }
        Ok(Self {
            x: parts[0],
            y: parts[1],
            z: parts[2],
        })
    }
}

fn solve_part1(connector: &mut ComponentConnector) -> u64 {
    for _ in 0..connector.positions.len().max(1000) {
        connector.make_shortest_connection();
    }

    // NOTE: clone component_sizes so sorting does not mutate ComponentConnector's state
    let mut component_sizes = connector.component_sizes.clone();
    component_sizes.sort();

    assert!(
        component_sizes.len() > 3,
        "should have at least 3 components"
    );

    component_sizes
        .into_iter()
        .rev()
        .take(3)
        .reduce(std::ops::Mul::mul)
        .expect("should have at least 3 components")
}

struct ComponentConnector {
    positions: Vec<Position>,
    uf: UnionFind,
    component_sizes: Vec<u64>,
    direct_connections: Vec<HashSet<usize>>,
}

impl ComponentConnector {
    fn new(positions: Vec<Position>) -> Self {
        Self {
            uf: UnionFind::default(),
            component_sizes: vec![1u64; positions.len()],
            direct_connections: vec![HashSet::<usize>::new(); positions.len()],
            positions,
        }
    }

    fn make_shortest_connection(&mut self) {
        let mut shortest_indirect_connection: Option<(usize, usize, u64)> = None;

        for (i1, p1) in self.positions.iter().enumerate() {
            let mut min_dist: Option<(usize, u64)> = None;
            let direct_connections_from_p1 = &self.direct_connections[i1];

            for i2 in i1 + 1..self.positions.len() {
                if direct_connections_from_p1.contains(&i2) {
                    continue;
                }

                // NOTE: use distance_squared since it behaves the same as regular distance
                // and saves us the sqrt operation
                let dist = p1.distance_squared(&self.positions[i2]);
                match min_dist {
                    Some((_, acc_min_dist)) => {
                        if dist < acc_min_dist {
                            min_dist = Some((i2, dist));
                        }
                    }
                    None => {
                        min_dist = Some((i2, dist));
                    }
                }
            }

            match (shortest_indirect_connection, min_dist) {
                (Some((_, _, acc_min_dist)), Some((i2, current_min_dist))) => {
                    if current_min_dist < acc_min_dist {
                        shortest_indirect_connection = Some((i1, i2, current_min_dist));
                    }
                }
                (None, Some((i2, current_min_dist))) => {
                    shortest_indirect_connection = Some((i1, i2, current_min_dist));
                }
                (_, _) => {}
            };
        }

        if let Some((i1, i2, dist)) = shortest_indirect_connection {
            self.direct_connections[i1].insert(i2);
            self.direct_connections[i2].insert(i1);

            // TODO: (perf) optimization possibility: do not `find` right before `union`.
            // `union` is doing its own `find` inside. Expose the "pre-union" ids from `union`.
            let component_id1 = self.uf.find(i1);
            let component_id1_size = self.component_sizes[component_id1];
            self.component_sizes[component_id1] = 0;

            let component_id2 = self.uf.find(i2);
            let component_id2_size = self.component_sizes[component_id2];
            self.component_sizes[component_id2] = 0;

            #[cfg(debug_assertions)]
            eprintln!(
                "Connecting {p1:^20} and {p2:^20} (components {:2} and {:2}), dist = {dist}",
                component_id1,
                component_id2,
                p1 = self.positions[i1],
                p2 = self.positions[i2],
            );

            let combined_id = self.uf.union(i1, i2);
            self.component_sizes[combined_id] = component_id1_size + component_id2_size;
        } else {
            unreachable!("no indirect connection could be made");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_connected_components() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let positions = parse_positions(input.lines());
        let mut connector = ComponentConnector::new(positions);

        for _ in 0..10 {
            connector.make_shortest_connection();
        }
        connector.component_sizes.sort();

        assert_eq!(
            connector
                .component_sizes
                .into_iter()
                .rev()
                .take(6)
                .collect::<Vec<_>>(),
            vec![5, 4, 2, 2, 1, 1]
        );
    }

    #[test]
    fn test_position_distance() {
        let p1: Position = "162,817,812".parse().unwrap();
        let p2: Position = "425,690,689".parse().unwrap();
        let p3: Position = "984,92,344".parse().unwrap();

        let d1 = p1.distance_squared(&p2);
        let d2 = p1.distance_squared(&p3);

        assert!(d1 < d2);
    }
}
