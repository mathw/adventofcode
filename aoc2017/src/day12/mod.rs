use petgraph::visit::DfsPostOrder;
use petgraph::Graph;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub fn go() {
    let input = include_str!("input.txt");

    let graph = parse_input(input);
}

fn parse_input(input: &str) -> Graph<u32, ()> {
    let lines = input.lines().filter_map(parse_input_line);

    let mut edges = Vec::new();
    let mut graph = Graph::new();

    for (node, links) in lines {
        graph.add_node(node);
        for link in links {
            edges.push((node, link));
        }
    }

    graph.extend_with_edges(edges);

    graph
}

fn parse_input_line(line: &str) -> Option<(u32, Vec<u32>)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+) <-> (.+)").unwrap();
    }

    let c = RE.captures(line);
    match c {
        Some(cap) => {
            let num = u32::from_str(&cap[1]);
            match num {
                Ok(n) => Some((
                    n,
                    (&cap[2])
                        .trim()
                        .split(", ")
                        .filter_map(|n| u32::from_str(n).ok())
                        .collect::<Vec<_>>(),
                )),
                Err(_) => None,
            }
        }
        None => None,
    }
}


#[cfg(test)]
mod tests {
    mod parse_input {
        use super::super::parse_input_line;

        #[test]
        fn parse_single_connection() {
            let input = "1 <-> 1";
            if let Some((node, connections)) = parse_input_line(input) {
                assert_eq!(node, 1);
                assert_eq!(connections, vec![1]);
            } else {
                assert!(false);
            }
        }
    }
}
