use std::{collections::HashMap, io::stdin};

fn main() {
    let graph = Graph::parse(
        stdin()
            .lines()
            .map(|line| line.expect("lines should be valid")),
    )
    .expect("input should be valid");

    println!("Part 1: {}", solve_part1(&graph));
}

fn solve_part1(graph: &Graph) -> u32 {
    todo!()
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct GraphNode {
    name: String,
    index: usize,
    neighbor_indexes: Vec<usize>,
}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
struct Graph {
    nodes: Vec<GraphNode>,
    node_indexes_by_name: HashMap<String, usize>,
}

impl Graph {
    fn insert_edges(&mut self, from: &str, neighbors: &[&str]) {
        let neighbor_indexes: Vec<usize> = neighbors
            .into_iter()
            .map(|name| self.get_node_index(name))
            .collect();
        let from_node = self.get_node_mut(from);

        for index in neighbor_indexes {
            from_node.neighbor_indexes.push(index);
        }
    }

    fn get_node_mut<'a>(&'a mut self, name: &str) -> &'a mut GraphNode {
        let index = self.get_node_index(name);
        self.nodes.get_mut(index).expect("node should exist")
    }

    fn get_node_index(&mut self, name: &str) -> usize {
        match self.node_indexes_by_name.get(name) {
            Some(&index) => index,
            None => {
                // The node will be inserted as last
                let index = self.nodes.len();
                let new_node = GraphNode {
                    name: name.to_string(),
                    index,
                    neighbor_indexes: Vec::new(),
                };

                self.nodes.push(new_node);
                self.node_indexes_by_name.insert(name.to_string(), index);
                index
            }
        }
    }

    fn parse(input: impl Iterator<Item = impl AsRef<str>>) -> Result<Self, String> {
        let mut graph = Self::default();

        for line in input {
            let line = line.as_ref();
            let (node, neighbors) =
                parse_line(line).map_err(|err| format!("cannot parse line {line}: {err:?}"))?;
            graph.insert_edges(node, &neighbors);
        }

        Ok(graph)
    }
}

// Parses a line like `ccc: ddd eee fff`
// into the root node (ccc) and a list of child nodes (ddd, eee, fff).
fn parse_line(line: &str) -> Result<(&str, Vec<&str>), String> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        return Err(format!("invalid line format: {line}"));
    }
    let root = parts[0].trim();
    let children: Vec<&str> = parts[1]
        .split_whitespace()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if children.is_empty() {
        return Err(format!("no children found for root: {root}"));
    }

    Ok((root, children))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_valid() {
        assert_eq!(
            parse_line("ccc: ddd eee fff"),
            Ok(("ccc", vec!["ddd", "eee", "fff"]))
        );

        assert!(
            matches!(parse_line("ccc ddd eee fff"), Err(_)),
            "Expected error for missing colon"
        );
        assert!(
            matches!(parse_line("ccc:"), Err(_)),
            "Expected error for no children"
        );
    }

    #[test]
    fn test_parse_graph() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

        let mut graph = Graph::parse(input.lines()).expect("input should be valid");

        assert_eq!(
            graph.nodes.len(),
            // NOTE: 10 nodes that have neighbors, and "out" that is the final node
            11
        );
        assert_eq!(graph.get_node_mut("out").neighbor_indexes, vec![]);
        assert_eq!(graph.get_node_mut("aaa").neighbor_indexes.len(), 2);
    }
}
