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
    struct Solver<'a> {
        paths_to_out_count: Vec<u32>,
        visited: Vec<bool>,
        graph: &'a Graph,
    }

    impl<'a> Solver<'a> {
        fn visit(&mut self, index: usize) {
            self.visited[index] = true;
            let node = &self.graph.nodes[index];

            for &neighbor_index in node.neighbor_indexes.iter() {
                if !self.visited[neighbor_index] {
                    self.visit(neighbor_index);
                }

                self.paths_to_out_count[index] += self.paths_to_out_count[neighbor_index];
            }
        }
    }

    let out_node_index = *graph
        .node_indexes_by_name
        .get("out")
        .expect("'out' node should exist");

    let mut paths_to_out_count = vec![0; graph.nodes.len()];
    paths_to_out_count[out_node_index] = 1;

    let mut visited = vec![false; graph.nodes.len()];
    visited[out_node_index] = true;

    let mut solver = Solver {
        graph,
        paths_to_out_count,
        visited,
    };

    let start_node_index = *graph
        .node_indexes_by_name
        .get("you")
        .expect("'you' node should exist");
    solver.visit(start_node_index);

    solver.paths_to_out_count[start_node_index]
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
            .map(|name| self.get_or_create_node_index(name))
            .collect();
        let from_node = self.get_node_mut(from);

        for index in neighbor_indexes {
            from_node.neighbor_indexes.push(index);
        }
    }

    fn get_node_mut<'a>(&'a mut self, name: &str) -> &'a mut GraphNode {
        let index = self.get_or_create_node_index(name);
        self.nodes.get_mut(index).expect("node should exist")
    }

    fn get_or_create_node_index(&mut self, name: &str) -> usize {
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

        let graph = Graph::parse(input.lines()).expect("input should be valid");

        assert_eq!(
            graph.nodes.len(),
            // NOTE: 10 nodes that have neighbors, and "out" that is the final node
            11
        );

        fn get_node<'a>(graph: &'a Graph, name: &str) -> Option<&'a GraphNode> {
            let index = *graph.node_indexes_by_name.get(name)?;
            graph.nodes.get(index)
        }
        assert_eq!(get_node(&graph, "out").unwrap().neighbor_indexes, vec![]);
        assert_eq!(get_node(&graph, "aaa").unwrap().neighbor_indexes.len(), 2);
    }

    #[test]
    fn test_solve_part1() {
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

        let graph = Graph::parse(input.lines()).expect("input should be valid");

        assert_eq!(solve_part1(&graph), 5);
    }
}
