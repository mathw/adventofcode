mod maze;
mod parse;

use petgraph::Graph;
use petgraph::algo::dijkstra;
use std::collections::HashSet;

pub fn do_day24(input: &str) {}


#[test]
fn test_sample_preparsed() {
    let mut graph = Graph::new_undirected();
    let zero = graph.add_node("0");
    let one = graph.add_node("1");
    let two = graph.add_node("2");
    let three = graph.add_node("3");
    let four = graph.add_node("4");
    graph.add_edge(zero, one, 2.0);
    graph.add_edge(zero, four, 2.0);
    graph.add_edge(one, two, 6.0);
    graph.add_edge(two, three, 2.0);
    graph.add_edge(four, three, 8.0);

    let mut visited = HashSet::new();
    let mut current = zero;
    // let mut paths = Vec::new();

    loop {
        visited.insert(current);
        let pathlengths = dijkstra(&graph, zero, None, |e| *e.weight());
        let mut r =
            pathlengths.iter().filter(|&(node, _)| !visited.contains(node)).collect::<Vec<_>>();

        if r.len() == 0 {
            break;
        }

        r.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

        println!("{:?}", r);

        let next = r[0].0;
        println!("Next node {:?}", next);

        current = *next;
    }

    assert!(false);
}

#[test]
fn test_parse_render() {
    let input = "###########
#0.1.....2#
#.#######.#
#4.......3#
###########
";

    let maze = parse::parse(input);
    let render = maze.render();
    assert_eq!(input, &render);
}
