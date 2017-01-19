mod node;
mod grid;

use std::str::FromStr;

use self::node::Node;
use self::grid::Grid;

pub fn do_day22(input: &str) {
    let nodes = input.lines().filter_map(|l| Node::from_str(l).ok()).collect::<Vec<Node>>();
    let mut grid = Grid::new();
    for node in nodes.iter().cloned() {
        grid.add_node(node);
    }

    println!("My grid has {} nodes", grid.size());
    // let mut pairs = Vec::new();
    // for node in nodes {
    //     let ps = grid.viable_pairs_with(&node);
    //     pairs.extend(ps.iter().cloned());
    // }
    //
    // println!("There are {} viable pairs in this grid", pairs.len());

    let target = grid.target_node_coordinates().unwrap();
    let target = grid.get_node_at(&target).unwrap();

    let empty_node = grid.get_empty_node().unwrap();

    println!("{}", grid.render_grid_symbolic(&target));
}
