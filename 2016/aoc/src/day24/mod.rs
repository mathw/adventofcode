mod maze;
mod parse;

use petgraph::Graph;
use petgraph::algo::dijkstra;

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

    let r = dijkstra(&graph, zero, None, |w| *w.weight());

    println!("{:?}", r);
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
